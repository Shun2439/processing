void draw_bin() {
  noFill();
  stroke(255);
  strokeWeight(2);
  beginShape();
  curveVertex(200, 300);
  curveVertex(200, 300);
  
  curveVertex(210, 310);
  curveVertex(210, 450);
  curveVertex(290, 450);
  curveVertex(290, 310);
  
  curveVertex(300, 300);
  curveVertex(300, 300);
  endShape();
}

void setup() {
  size(500, 500);
}

void draw() {
  draw_bin();
}
