const { promisify } = require('util');
const jimp = require('jimp');

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
    path,
    width,
    height,
    CROP_X,
    CROP_Y,
    CROP_WIDTH,
    CROP_HEIGHT,
    COLOR_PRECISION,
}) => {
    const image = (await promisify(jimp.read)(path));
    const resized = image.crop(
        CROP_X,
        CROP_Y,
        CROP_WIDTH || image.getWidth(),
        CROP_HEIGHT || image.getHeight(),
    )
    .resize(width,height);

    let output = []

    for(let x = 0; x < width; x++){
        for(let y = 0; y < height; y++){
            const int = resized.getPixelColor(x,y);
            const color = flattenColor(jimp.intToRGBA(int),COLOR_PRECISION);
            output.push([x,y,color.r,color.g,color.b].join(','));
        }
    }

    return output.join(':')
}
