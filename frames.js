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
    CROP_X,
    CROP_Y,
    CROP_WIDTH,
    CROP_HEIGHT,
    COLOR_PRECISION,
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
                return flattenColor(jimp.intToRGBA(image.getPixelColor(x,y)),COLOR_PRECISION);
            }
            function send(){
                return output.push([x,y,Object.values(getColor(currImage))].join(','));
            }
            if(prevImage && nextImage){
                if(!equal(getColor(currImage),getColor(prevImage),getColor(nextImage))) send();
            } else if(prevImage || nextImage){
                if(!equal(getColor(currImage),getColor(prevImage||nextImage))) send();
            } else send();
        }
    }

    return output.join(':')
}
