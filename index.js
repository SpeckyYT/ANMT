const filehound = require('filehound');
const path = require('path');
const cp = require('child_process');
const fs = require('fs');
const jimp = require('jimp');
const Piscina = require('piscina');
const { ffprobe_simple } = require('js-ffmpeg');

const COLOR_PRECISION = 1; // 1 best quality (more objects) | 8 worst quality (less objects)
const CROP_X = 0;   // all 0 for disable
const CROP_Y = 0;
const CROP_WIDTH = 0;
const CROP_HEIGHT = 0;
const OPTIMISE_FIRST_FRAME = false;

const MAX_PIXELS = 999;     // 999 is the max amount of usable colors in 2.1
const FILENAMENUMBERS = 5;  // e.g. filename00001.png
const SKIP_EXTRACTING = false;
const SKIP_PROCESSING = false;

const createFolder = (folder) => {
    if(!fs.existsSync(folder)) return fs.mkdirSync(folder);
}

const videosFolder = path.join(process.cwd(),'videos');
createFolder(videosFolder);

const files = filehound.create()
.path(videosFolder)
.ext([
    'mp4',
    'avi',
    'mov',
    'wmv',
    'flv',
    'gif',
    'apng',
])
.depth(0)
.findSync();

const betaJS = (prom) => {
    return new Promise((res,rej) => {
        prom.success(res)
        prom.error(rej)
    });
}

(async function(){
    const colorPrecision = 2**Math.max(Math.min(COLOR_PRECISION,8),1)-1
    const promises = []
    for(const filePath of files){
        const fileData = path.parse(filePath);
        fileData.name = fileData.name.replace(/[^a-zA-Z0-9_-]+/,'') || 'speckywashere';
        const framesFolder = path.join(videosFolder,'frames');
        createFolder(framesFolder);
        const thisFramesFolder = path.join(framesFolder,fileData.name);
        createFolder(thisFramesFolder);

        function log(name,description){
            console.log(`[${name.toUpperCase()}] (${fileData.name}): ${description}`);
        }

        promises.push(
            new Promise(res => {
                if(SKIP_EXTRACTING) return res();
                log('extracting','start');
                cp.spawn(
                    'ffmpeg',
                    [
                        '-i',
                        filePath,
                        path.join(
                            thisFramesFolder,
                            `${fileData.name}%${`${FILENAMENUMBERS}`.padStart(2,'0')}d.png`
                        )
                    ],
                    {
                        stdio: ['inherit','inherit','inherit']
                    }
                )
                .on('close', () => {
                    log('extracting','done');
                    res()
                })
            })
            .then(async () => {
                const frames = filehound.create()
                .path(thisFramesFolder)
                .ext('png')
                .depth(0)
                .findSync();

                if(!frames.length) throw "No frames found";

                const firstFrame = await jimp.read(frames[0]);

                const originalWidth = (CROP_WIDTH||firstFrame.getWidth()) - CROP_X;
                const originalHeight = (CROP_HEIGHT||firstFrame.getHeight()) - CROP_Y;
                const originalPixels = originalWidth * originalHeight;
                const ratio = originalWidth / originalHeight;
                const scale = Math.sqrt(originalPixels / MAX_PIXELS);
                global.width = Math.floor(ratio * originalHeight / scale);
                global.height = Math.floor(originalHeight / scale);
                // ^ this is not optimal, but it works fine atm

                if(SKIP_PROCESSING) return;
                log('preprocessing','start')
                const pool = new Piscina({
                    filename: path.join(process.cwd(),'frames.js'),
                });
                const promises = []
                for(let frame = 0; frame < frames.length; frame++){
                    promises.push(
                        pool.run(
                            {
                                path: frames[frame],
                                previousPath: frames[frame-1],
                                width: global.width,
                                height: global.height,
                                CROP_X,
                                CROP_Y,
                                CROP_HEIGHT,
                                CROP_WIDTH,
                                COLOR_PRECISION: colorPrecision,
                                OPTIMISE_FIRST_FRAME,
                            }
                        )
                        .then(data => {
                            log('preprocessing',`frame #${frame} done`)
                            return data;
                        })
                    );
                }
                return Promise.all(promises)
                .then(data => {
                    log('preprocessing','done')
                    return data
                })
            })
            .then(async (frames=[]) => {
                if(SKIP_PROCESSING) return;

                log('writing','start')
                const videoData = await betaJS(ffprobe_simple(filePath));
                const fps = (videoData.video.frames || frames.length) / videoData.duration;
                const outputFolder = path.join(videosFolder,'output');
                createFolder(outputFolder);
                const outputFile = path.join(outputFolder,`${fileData.name}.txt`);
                frames.unshift(
                    [
                        global.width,
                        global.height,
                        fps
                    ]
                    .join(',')
                )
                fs.writeFileSync(outputFile,'');
                for(const frame of frames){
                    fs.appendFileSync(outputFile,`${frame}\n`);
                }
                log('writing','done')
            })
            .then(async () => {
                const outputFolder = path.join(videosFolder,'output');
                createFolder(outputFolder);
                const outputFile = path.join(outputFolder,`${fileData.name}.txt`);

                const file = fs.readFileSync(outputFile,{encoding:'utf-8'});
                const lines = file.split(/\r?\n/g);

                let triggers = 0;

                const [ width, height, fps_f ] = lines[0].split(',').map(n => parseFloat(n));
                for(let index = 1; index < lines.length; index++){
                    triggers += lines[index].split(':').filter(v => v).length
                }

                const pixels = width * height;
                const fps = parseFloat(fps_f).toFixed(3);
                const total = pixels + triggers;

                log('counting',`${fps} frames per second`)
                log('counting',`${width * height} pixels (${width}x${height})`)
                log('counting',`${triggers} color triggers`)
                log('counting',`${total} minimum total objects`)
            })
        )
    }
    await Promise.all(promises);
})()
