const { promisify } = require('util');
const jimp = require('jimp');
const read = promisify(jimp.read);
const deepEqual = require('deep-equal');

function equal(...objs){
    return !objs.some((obj,i,arr) => i > 0 && !deepEqual(obj,arr[i-1]))
}

function flattenNumber(number,colorRound){
    return Math.round(number/colorRound)*colorRound;
}

function flattenColor(color,colorRound){
    const newColor = {}
    newColor.r = flattenNumber(color.r,colorRound)
    newColor.g = flattenNumber(color.g,colorRound)
    newColor.b = flattenNumber(color.b,colorRound)
    return newColor
}

module.exports = async ({
    currentPath,
    previousPath,
    nextPath,
    width,
    height,
    index,
    length,
    CROP_X,
    CROP_Y,
    CROP_WIDTH,
    CROP_HEIGHT,
    COLOR_PRECISION,
    OPTIMIZE_PREVIOUS_FRAME,
    OPTIMIZE_NEXT_FRAME,
}) => {
    const readImage = async path =>
        path && (await read(path)).crop(CROP_X, CROP_Y,CROP_WIDTH, CROP_HEIGHT).resize(width,height)

    const currImage = await readImage(currentPath);
    const prevImage = await readImage(previousPath);
    const nextImage = await readImage(nextPath);

    const output = []

    for(let x = 0; x < width; x++){
        for(let y = 0; y < height; y++){
            function getColor(image){
                return flattenColor(
                    jimp.intToRGBA(
                        image ?
                            image.getPixelColor(x,y) : 0
                    ), COLOR_PRECISION
                );
            }
            function send(){
                return output.push([x,y,Object.values(getColor(currImage))].join(','));
            }
            if(OPTIMIZE_PREVIOUS_FRAME && index > 0 && !equal(getColor(currImage),getColor(prevImage))){
                send();
            }else if(OPTIMIZE_NEXT_FRAME && index < length-1 && !equal(getColor(currImage),getColor(nextImage))){
                send();
            }else if(OPTIMIZE_PREVIOUS_FRAME && index == 0 || OPTIMIZE_NEXT_FRAME && index == length-1){
                send();
            }
        }
    }

    return output.join(':')
}
