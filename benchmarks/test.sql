.load target/release/libimg0
.timer on

create table images as 
  select readfile('me.jpg') as image;

select img_width(image) from images;