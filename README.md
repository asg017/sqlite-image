# sqlite-img

A work-in-progress SQLite extension for converting, resizing, and manipulating images! Not meant to be widely shared.

Once it's ready, you'll be able to do things like:

```sql
.load ./img0

-- resize image_0001.jpg to 200x200, then write to thumbnail.jpg
select writefile(
  'thumbnail.jpg',
  img_resize(
    readfile('image_0001.jpg'),
    '200x200'
  )
);

-- rotate image_0001.jpg 90 degrees and write to rotated.jpg
select writefile(
  'rotated.jpg',
  img_rotate90(readfile('image_0001.jpg'))
);

-- convert image_0001.jpg and write to image_0001.png
select writefile(
  'image_0001.png',
  img_as_png(readfile('image_0001.jpg'))
);
```
