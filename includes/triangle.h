#ifndef REPR_TRIANGLE_H
#define REPR_TRIANGLE_H

#include <stdint.h>
#include <stdbool.h>

#define VERTEX_COUNT 3
#define DEGREE_CONVERSION_CONSTANT (3.14 / 180.0)

typedef struct {
  float x;
  float y;
} Point;

typedef struct {
  float radius;
  float rot_angle;
  Point* centre;
  Point** points;
  float rot_angle_change;
  int32_t area;
} Triangle;

Point* PointNew(float x, float y);
Triangle* TriangleNew(float display_width, float display_height);
Point* RotateLine(float start_x, float start_y, float length, float rot_angle);
Point** GetVertices(Triangle* tri);
bool DoesTriangleCollideWithPoint(Triangle* tri, Point* point);
bool FindAngleOfRefraction(float angle_of_incidence);
Point** FindNearestVertex(Triangle* tri, Point* pos);
bool IsPointInsideOfLine(float x1, float y1, float x2, float y2, float x, float y);
Point** FondNormal(Point** line);
float FindAngleBetweenLines(Point*** lines);
// collide
void TriangleDraw(Triangle* tri);

#endif