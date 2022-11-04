.load target/release/libimg0
.timer on

.header on
.mode box

.echo on

with images as (
  select readfile('me.jpg') as image
)
select writefile(
  'overlay-all.jpg',
  img_overlay_all(
    image,
    img_flip_vertical(
      img_grayscale(
        img_resize(image, 100, 100)
      )
    ), 100, 100,
    img_flip_horizontal(
      img_blur(
        img_resize(image, 200, 200), 
        2
      )
    ), 250, 100,
    img_rotate90(img_resize(image, 50, 50)), 0, 0,
    img_rotate180(img_resize(image, 50, 50)), 0, 50,
    img_rotate270(img_resize(image, 50, 50)), 0, 100,
    img_solid_rgb(50, 50, 200, 0, 0), 0, 150,
    img_solid_rgb(50, 50, 0, 200, 0), 0, 200,
    img_solid_rgb(50, 50, 0, 0, 200), 0, 250,
    img_rect(50, 50), 0, 300
  )
)
from images;

.exit

select writefile(
  'all.jpg',
  img_replace(
    img_replace(
      readfile('me.jpg'),
      img_flip_vertical(
        img_grayscale(
          img_resize(
            readfile('me.jpg'), 
            100, 
            100, 
            'nearest'
          )
        )
      ),
      100, 100
    ),
    img_flip_horizontal(
      img_blur(
        img_resize(
          readfile('me.jpg'), 
          200, 
          200, 
          'nearest'
        ), 
        2
      )
    ),
    250,
    100
  )
);


.exit

select writefile(
  'resized.jpg',
  img_resize(readfile('me.jpg'), 100, 100, "nearest")
);

select writefile(
  'thumb.jpg',
  img_thumbnail(readfile('me.jpg'), 100, 100)
);

.exit

.load /Users/alex/tmp/crypto
--select distinct hex(sha1(img_grayscale(readfile('me.jpg')))) from generate_series(1,100);

select writefile('test.jpg', 
  img_replace(
    readfile('me.jpg'),
    img_grayscale(readfile('me.jpg')),
    200, 
    200
  )
);

.exit

select writefile('gray.jpg', img_grayscale(readfile('me.jpg')));
select writefile('blur_gray_fs.jpg', img_blur(readfile('gray.jpg'), 2));
select writefile('blur_gray_memory.jpg', img_blur(img_grayscale(readfile('me.jpg')), 2));
.exit

select writefile(
  'test.jpg',
  img_grayscale(readfile('me.jpg'))
) as gray_only;

select writefile(
  'test.jpg',
  img_blur(readfile('me.jpg'), 2)
) as blur_only;

-- "Format error decoding Jpeg: invalid JPEG format: FF 00 found where marker was expected"
select writefile(
  'test.jpg',
    img_blur(
      img_grayscale(readfile('me.jpg')), 
      2
    )
) as gray_then_blur;

-- "Format error decoding Jpeg: invalid JPEG format: encountered table with excessive length in DHT"
select writefile(
  'test.jpg',
  img_grayscale(
    img_blur(readfile('me.jpg'), 2)
  )
) as blur_then_gray;
.exit

create table images as 
  select readfile('me.jpg') as image;

select img_width(image) from images;
--select img_crop(image) from images;

select writefile(
  'me.png',
  img_as_png(readfile('me.jpg'))
);

select writefile(
  'me2.jpg',
  img_as_jpeg(readfile('me.jpg'), 70)
);

select writefile(
  'crop.jpg',
  img_crop(readfile('me.jpg'), 150, 150, 100, 100)
);

select writefile(
  'crop.png',
  img_crop_png(readfile('me.jpg'), 150, 150, 100, 100)
);


select writefile(
  'crop-grayscale.jpg',
  img_grayscale(
    img_crop_jpeg(readfile('me.jpg'), 150, 150, 100, 100)
  )
);


select writefile(
  'grayscale.jpg',
  img_grayscale(readfile('me.jpg'))
);