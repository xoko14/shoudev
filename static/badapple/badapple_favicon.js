(async () => {
  let data = await fetch("https://shoudev.com/badapple/badapple16_5.ba");
  let binary = await data.bytes();
  let pointer = 0;

  function getU8() {
    let byte = binary[pointer]
    pointer++;
    return byte;
  }

  function getU16() {
    let num = binary[pointer] + (binary[pointer + 1] << 8);
    pointer += 2;
    return num;
  }

  function getU32() {
    let num = binary[pointer] + (binary[pointer + 1] << 8) + (binary[pointer + 2] << 16) + (binary[pointer + 3] << 24);
    pointer += 4;
    return num;
  }

  let height = getU8();
  let width = getU8();
  let framerate = getU8();
  let frameCount = getU32();
  let frametime = 1000/framerate;

  let resetpointer = pointer;
  
  var link = document.querySelector("link[rel~='icon']");
  if (!link) {
    link = document.createElement('link');
    link.rel = 'icon';
    link.sizes = 'any';
    link.type= 'image/svg+xml'
    document.head.appendChild(link);
  }

  function drawFrame() {
    if(pointer>= binary.length)
      pointer = resetpointer;

    let svg = `<svg width="16" height="16" viewbox="0 0 16 16" xmlns="http://www.w3.org/2000/svg" version="1.1">`;

    let sectionCount = getU16();

    let x = 0;
    let y = 0;

    for (let i = 0; i < sectionCount; i++) {
      let color = getU8() == 0 ? "black" : "white";
      let pixelCount = getU16();
      for (let j = 0; j < pixelCount; j++) {

        svg += `<rect fill="${color}" x="${x}" y="${y}" width="1" height="1"/>`

        if (++x >= width) {
          y++;
          x = 0;
        }
      }
    }

    svg += "</svg>"


    let url = `data:image/svg+xml,${encodeURIComponent(svg)}`

    link.href = url;

    setTimeout(drawFrame, frametime);
  }

  drawFrame()


})()
