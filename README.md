Exploring game of life inspired simulation.

This is the color map that will be used as the environment for entities on top. The color will decide if a particular entity dies or lives.

This color transition is obtained using bilinear interpolation: every cell is delimited by 4 points, a point has (x,y) coordinate and a value of temperature.

The equation used: `T(x,y) = a + b·x + c·y + d·x·y`
a,b,c,d are computed starting from the 4 cells point using A^-1 dot (t1, t2, t3, t4)

Where A is the matrix of coefficients for a,b,c,d rispectively, so for example first row is:
- point1: [1, x1, y1, x1 * y1]  

<img width="1401" height="1393" alt="image_2025-10-26_21-19-28" src="https://github.com/user-attachments/assets/833578c0-e8a1-4879-93dc-bda0c83d4d71" />
<img width="1401" height="1393" alt="image_2025-10-26_21-20-24" src="https://github.com/user-attachments/assets/1ea0cfec-2458-4233-bdf7-bd106c69fa89" />
<img width="1020" height="1023" alt="image_2025-10-26_21-32-06" src="https://github.com/user-attachments/assets/e97ae47b-df17-41ad-99aa-06dc7b6b1221" />
<img width="1263" height="1285" alt="image_2025-10-26_21-28-07" src="https://github.com/user-attachments/assets/2471691c-8f4a-40dd-900d-697ce2319364" />
<img width="1150" height="1122" alt="image_2025-10-26_21-10-27" src="https://github.com/user-attachments/assets/cebc5d8d-e3a8-4c7b-9698-b8d4effc7bda" />
<img width="1327" height="1324" alt="image_2025-10-26_21-11-47" src="https://github.com/user-attachments/assets/716004f4-57c3-4ae0-a233-12f325870010" />
