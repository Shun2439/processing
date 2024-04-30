int text_x = 100, text_y = 200;
int move_dir_x = 2, move_dir_y = 3;
int symbol_color_r = 0, symbol_color_g = 50, symbol_color_b = 100;
int change_color_r = 2, change_color_g = 1, change_color_b = 3;

void setup() {
  size(500, 500);
  textSize(70);
  textAlign(CENTER, CENTER);
}

void draw() {
  background(0);
  fill(symbol_color_r, symbol_color_g, symbol_color_b);
  ellipse(text_x, text_y, 150, 100);
  fill(0);
  text("DVD", text_x, text_y);

  symbol_color_r+=change_color_r;
  symbol_color_g+=change_color_g;
  symbol_color_b+=change_color_b;

  if (symbol_color_r >= 255 || symbol_color_r <= 0) change_color_r *= -1;
  if (symbol_color_g >= 255 || symbol_color_g <= 0) change_color_g *= -1;
  if (symbol_color_b >= 255 || symbol_color_b <= 0) change_color_b *= -1;

  // テキストを動かす
  text_x += move_dir_x;
  text_y += move_dir_y;

  // 跳ね返りの処理
  if (text_x >= 425 || text_x <= 75) move_dir_x *= -1;
  if (text_y >= 450 || text_y <= 50) move_dir_y *= -1;
}
