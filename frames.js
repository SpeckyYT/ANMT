const { promisify } = require('util');
const jimp = require('jimp');
const read = promisify(jimp.read);

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

function areEqual(obj1,obj2){
    return !Object.entries(obj1)
    .some(v => obj2[v[0]] != v[1])
}

module.exports = async ({
    path,
    previousPath,
    width,
    height,
    CROP_X,
    CROP_Y,
    CROP_WIDTH,
    CROP_HEIGHT,
    COLOR_PRECISION,
}) => {
    const image = await read(path);
    const resized = image.crop(
        CROP_X,
        CROP_Y,
        CROP_WIDTH || image.getWidth(),
        CROP_HEIGHT || image.getHeight(),
    )
    .resize(width,height);

    const prevImage = previousPath && await read(previousPath);
    const prevResized = prevImage && prevImage.crop(
        CROP_X,
        CROP_Y,
        CROP_WIDTH || prevImage.getWidth(),
        CROP_HEIGHT || prevImage.getHeight(),
    )
    .resize(width,height);

    let output = []

    for(let x = 0; x < width; x++){
        for(let y = 0; y < height; y++){
            function send(){
                output.push([x,y,color.r,color.g,color.b].join(','));
            }
            const int = resized.getPixelColor(x,y);
            const color = flattenColor(jimp.intToRGBA(int),COLOR_PRECISION);
            if(prevResized){
                const prevInt = prevResized.getPixelColor(x,y);
                const prevColor = flattenColor(jimp.intToRGBA(prevInt),COLOR_PRECISION);
                if(!areEqual(color,prevColor)) send();
            }else{
                if(!areEqual(color,{r:255,g:255,b:255})) send();
            }
        }
    }

    return output.join(':')
}
