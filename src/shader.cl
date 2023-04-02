__kernel void nbody(__global float *buffer, __global float *positions) {
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

    float r =
        max(sqrt((dx * dx) + (dy * dy) + (dz * dz)), (float)SMOOTH_LENGTH);
    float r_inv = 1.0 / (r * r * r);

    buffer[(n * 3) + 0] += r_inv * dx * GRAV_STRENGTH;
    buffer[(n * 3) + 1] += r_inv * dy * GRAV_STRENGTH;
    buffer[(n * 3) + 2] += r_inv * dz * GRAV_STRENGTH;
  }
}
