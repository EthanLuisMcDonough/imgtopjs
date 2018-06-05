var img = (function() {
    var img = createGraphics($WIDTH, $HEIGHT, JAVA2D);
    var data = $DATA.split(/([ÿ-ǽ][a-z\d]+)/);
    var index = 0;
    img.loadPixels();
    data.forEach(function(seq) {
        if (seq === "") { return; }
        var value = seq[0].charCodeAt(0) - 255, count = +("0x" + seq.substr(1));
        for (var i = 0; i < count; i++) {
            img.imageData.data[index++] = value;
        }
    });
    img.updatePixels();
    return img;
})();
background(0);
imageMode(CENTER);
image(img, 200, 200);