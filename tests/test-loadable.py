import sqlite3
import unittest
import time
import os
from PIL import Image

EXT_PATH="./dist/debug/img0"

def connect(ext):
  db = sqlite3.connect(":memory:")

  db.execute("create table base_functions as select name from pragma_function_list")
  db.execute("create table base_modules as select name from pragma_module_list")

  db.enable_load_extension(True)
  db.load_extension(ext)

  db.execute("create temp table loaded_functions as select name from pragma_function_list where name not in (select name from base_functions) order by name")
  db.execute("create temp table loaded_modules as select name from pragma_module_list where name not in (select name from base_modules) order by name")

  db.row_factory = sqlite3.Row
  return db


db = connect(EXT_PATH)

def explain_query_plan(sql):
  return db.execute("explain query plan " + sql).fetchone()["detail"]

def execute_all(sql, args=None):
  if args is None: args = []
  results = db.execute(sql, args).fetchall()
  return list(map(lambda x: dict(x), results))

FUNCTIONS = [
  'img_as_jpeg',
  'img_as_png',
  'img_as_webp',
  'img_blur',
  'img_crop',
  'img_crop_jpeg',
  'img_crop_png',
  'img_debug',
  'img_flip_horizontal',
  'img_flip_vertical',
  'img_grayscale',
  'img_overlay_all',
  'img_rect',
  'img_replace',
  'img_resize',
  'img_resize',
  'img_rotate180',
  'img_rotate270',
  'img_rotate90',
  'img_solid_rgb',
  'img_thumbnail',
  'img_version',
  'img_width'
]

MODULES = []

ME_JPG = open('me.jpg', 'rb').read()
ME_PNG = open('me.png', 'rb').read()

from io import BytesIO


def pil_image(image):
  return Image.open(BytesIO(image))

class TestImg(unittest.TestCase):
  def test_funcs(self):
    funcs = list(map(lambda a: a[0], db.execute("select name from loaded_functions").fetchall()))
    self.assertEqual(funcs, FUNCTIONS)

  def test_modules(self):
    modules = list(map(lambda a: a[0], db.execute("select name from loaded_modules").fetchall()))
    self.assertEqual(modules, MODULES)
    
  def test_img_version(self):
    version = 'v0.1.0'
    self.assertEqual(db.execute("select img_version()").fetchone()[0], version)
  
  def test_img_debug(self):
    debug = db.execute("select img_debug()").fetchone()[0]
    self.assertEqual(len(debug.splitlines()), 2)



  def test_img_as_jpeg(self):
    img_as_jpeg = lambda x: db.execute("select img_as_jpeg(?, .9)", [x]).fetchone()[0]
    jpeg = pil_image(img_as_jpeg(ME_PNG))
    self.assertEqual(jpeg.format, "JPEG")
    self.assertEqual(jpeg.width, 460)
    self.assertEqual(jpeg.height, 460)

  def test_img_as_png(self):
    img_as_png = lambda x: db.execute("select img_as_png(?)", [x]).fetchone()[0]
    png = pil_image(img_as_png(ME_JPG))
    self.assertEqual(png.format, "PNG")
    self.assertEqual(png.width, 460)
    self.assertEqual(png.height, 460)

  def test_img_as_webp(self):
    img_as_webp = lambda x: db.execute("select img_as_webp(?)", [x]).fetchone()[0]
    
    webp = pil_image(img_as_webp(ME_JPG))
    self.assertEqual(webp.format, "WEBP")
    self.assertEqual(webp.width, 460)
    self.assertEqual(webp.height, 460)
    self.assertEqual(pil_image(img_as_webp(ME_PNG)).format, "WEBP")


  def test_img_blur(self):
    img_blur = lambda x: db.execute("select img_blur(?)", [x]).fetchone()[0]
    self.skipTest("TODO")

  def test_img_crop(self):
    img_crop = lambda x: db.execute("select img_crop(?)", [x]).fetchone()[0]
    self.skipTest("TODO")

  def test_img_crop_jpeg(self):
    img_crop_jpeg = lambda x: db.execute("select img_crop_jpeg(?)", [x]).fetchone()[0]
    self.skipTest("TODO")

  def test_img_crop_png(self):
    img_crop_png = lambda x: db.execute("select img_crop_png(?)", [x]).fetchone()[0]
    self.skipTest("TODO")

  def test_img_flip_horizontal(self):
    img_flip_horizontal = lambda x: db.execute("select img_flip_horizontal(?)", [x]).fetchone()[0]
    self.skipTest("TODO")

  def test_img_flip_vertical(self):
    img_flip_vertical = lambda x: db.execute("select img_flip_vertical(?)", [x]).fetchone()[0]
    self.skipTest("TODO")

  def test_img_grayscale(self):
    img_grayscale = lambda x: db.execute("select img_grayscale(?)", [x]).fetchone()[0]
    self.skipTest("TODO")

  def test_img_overlay_all(self):
    img_overlay_all = lambda x: db.execute("select img_overlay_all(?)", [x]).fetchone()[0]
    self.skipTest("TODO")

  def test_img_rect(self):
    img_rect = lambda x: db.execute("select img_rect(?)", [x]).fetchone()[0]
    self.skipTest("TODO")

  def test_img_replace(self):
    img_replace = lambda x: db.execute("select img_replace(?)", [x]).fetchone()[0]
    self.skipTest("TODO")

  def test_img_resize(self):
    img_resize = lambda img, w, h: db.execute("select img_resize(?, ?, ?)", [img, w, h]).fetchone()[0]
    resized = pil_image(img_resize(ME_JPG, 100, 100))

    self.assertEqual(resized.width, 100)
    self.assertEqual(resized.height, 100)
    self.assertEqual(resized.format, 'JPEG')

  def test_img_rotate180(self):
    img_rotate180 = lambda x: db.execute("select img_rotate180(?)", [x]).fetchone()[0]
    self.skipTest("TODO")

  def test_img_rotate270(self):
    img_rotate270 = lambda x: db.execute("select img_rotate270(?)", [x]).fetchone()[0]
    self.skipTest("TODO")

  def test_img_rotate90(self):
    img_rotate90 = lambda x: db.execute("select img_rotate90(?)", [x]).fetchone()[0]
    self.skipTest("TODO")

  def test_img_solid_rgb(self):
    img_solid_rgb = lambda x: db.execute("select img_solid_rgb(?)", [x]).fetchone()[0]
    self.skipTest("TODO")

  def test_img_thumbnail(self):
    img_thumbnail = lambda x: db.execute("select img_thumbnail(?)", [x]).fetchone()[0]
    self.skipTest("TODO")
    

  def test_img_width(self):
    img_width = lambda x: db.execute("select img_width(?)", [x]).fetchone()[0]
    self.assertEqual(
      img_width(ME_JPG),
      460
    )

class TestCoverage(unittest.TestCase):                                      
  def test_coverage(self):                                                      
    test_methods = [method for method in dir(TestImg) if method.startswith('test_')]
    funcs_with_tests = set([x.replace("test_", "") for x in test_methods])
    
    for func in FUNCTIONS:
      self.assertTrue(func in funcs_with_tests, f"{func} does not have corresponding test in {funcs_with_tests}")
    
    for module in MODULES:
      self.assertTrue(module in funcs_with_tests, f"{module} does not have corresponding test in {funcs_with_tests}")

if __name__ == '__main__':
    unittest.main()