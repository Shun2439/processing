int[][] empty_bin_vec = {
  {200, 300},
  {210, 310},
  {210, 450},
  {290, 450},
  {290, 310},
  {300, 300},
};

void draw_bin() {
  noFill();
  stroke(255);
  strokeWeight(2);
  beginShape();
  curveVertex(200, 300);
  for (int i = 0; i < empty_bin_vec.length; i++) for(int j = 0; j < empty_bin_vec[i].length - 1; j++) curveVertex(empty_bin_vec[i][j], empty_bin_vec[i][j + 1]);
  curveVertex(300, 300);
  endShape();
}

void setup() {
  size(500, 500);
}

void draw() {
  draw_bin();
}
