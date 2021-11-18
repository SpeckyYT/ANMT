const sharp = require('sharp');
const deepEqual = require('deep-equal');

function equal(...objs){
    return !objs.some((obj,i,arr) => i > 0 && !deepEqual(obj,arr[i-1]))
}

function flattenNumber(number,colorRound){
    return Math.round(number/colorRound)*colorRound;
}

function flattenColor(color,colorRound){
    return color && {
        r: flattenNumber(color.r,colorRound),
        g: flattenNumber(color.g,colorRound),
        b: flattenNumber(color.b,colorRound),
    }
}

function getColor(image,offset){
    return image && {
        r: image[offset],
        g: image[offset+1],
        b: image[offset+2],
    }
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
    const readImage = async path => {
        return sharp(path)
        .extract({
            left: CROP_X,
            top: CROP_Y,
            width: CROP_WIDTH,
            height: CROP_HEIGHT,
        })
        .resize(width, height, { kernel: 'cubic' })
        .ensureAlpha()
    }

    const [
        currImage,
        prevImage,
        nextImage,
    ] = await Promise.all(
        [
            currentPath,
            OPTIMIZE_PREVIOUS_FRAME && previousPath,
            OPTIMIZE_NEXT_FRAME && nextPath,
        ].map(async p => p && [...await (await readImage(p)).raw().toBuffer()])
    );

    const output = []

    for(let x = 0; x < width; x++){
        for(let y = 0; y < height; y++){
            const offset = x * 4 + y * width * 4

            const currColor = flattenColor(getColor(currImage,offset),COLOR_PRECISION)
            const prevColor = flattenColor(getColor(prevImage,offset),COLOR_PRECISION)
            const nextColor = flattenColor(getColor(nextImage,offset),COLOR_PRECISION)

            if(
                (prevImage && !equal(currColor,prevColor)) ||
                (nextImage && !equal(currColor,nextColor)) ||
                (OPTIMIZE_PREVIOUS_FRAME && index == 0) ||
                (OPTIMIZE_NEXT_FRAME && index + 1 == length)
            ) output.push([x,y,Object.values(currColor)].join(','));
        }
    }

    return output.join(':')
}
