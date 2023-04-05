__kernel void nbody(__global float *acceleration, __global float *positions) {
  int n = get_global_id(0);

  float x = positions[n * 3];
  float y = positions[(n * 3) + 1];
  float z = positions[(n * 3) + 2];

  for (int m = 0; m < NUM_PARTICLES; ++m) {
    if (n == m) {
      continue;
    }

    float dx = positions[m * 3] - x;
    float dy = positions[(m * 3) + 1] - y;
    float dz = positions[(m * 3) + 2] - z;

    float r_sq = ((dx * dx) + (dy * dy) + (dz * dz)) +
                 ((float)SMOOTH_LENGTH * (float)SMOOTH_LENGTH);
    float r = sqrt(r_sq);
    float s = GRAV_STRENGTH / (r * r_sq);

    acceleration[(n * 3) + 0] += dx * s;
    acceleration[(n * 3) + 1] += dy * s;
    acceleration[(n * 3) + 2] += dz * s;
  }
}
