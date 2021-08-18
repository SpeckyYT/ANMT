# Video2GD

Converts videos to GD!

## Disclaimer

| You're responsible for your own actions.
|-
| The developers of this project aren't in charge for the end-users' actions.

## Setup

You will require the following software to be installed on your machine before continuing:

- [ ] [Node.js](<https://nodejs.org/en/download/>)
  - [ ] run `npm i` in the current folder
- [ ] [FFmpeg](<https://www.ffmpeg.org/download.html>)
  - [ ] add FFmpeg to the environment variables
- [ ] [SPWN](<https://github.com/spu7Nix/SPWN-language/releases>)
  - [ ] download the modules imported in `index.spwn`

## Load images

In the current folder, create a new folder named `videos`.
In that folder, add the videos you want to process (works best one at a time).
Be sure that the videos you provide have a simple alphanumeric name.

Note: The longer the videos are, the longer it will take to process and compile them.

## Process images

Optionally, check out `index.js` and edit the `COLOR_PRECISION` value.

Run `node .` and wait until it's done.
This will extract all the frames from your video and will create a new file in `videos/output/`.

If you only want to update the output file or only extract the frames, change `SKIP_EXTRACTING` or `SKIP_PROCESSING` in `index.js` to true.

## Compile into Geometry Dash

Check out `videos/output/`, and look for the file you generated.
Edit the variable `filename` in `index.spwn` to the name of the desired file in `videos/output/` (without `.txt`).
Now you can run `spwn build index.spwn`, and slowly but surely it will build your video to your latest Geometry Dash level.

## FAQ

### How do I make the program stop using a file in the `videos` folder?

Create a sub-folder named in any way and put the video there.
Otherwise, you can delete it, or remove it from the videos folder.

## Supported video/animation formats

- [x] mp4
- [x] avi
- [x] mov
- [x] wmv
- [x] flv
- [x] gif
- [x] apng

```cmd
ffmpeg -formats
```

Other formats that are demuxing-supported by FFmpeg can be added to the list.
