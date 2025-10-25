
= Debug
1. il metodo find_nearest_cell restituisce delle celle sbagliate
  - ad una traslazione sulle x del cubo, la nearest_cell varia anche in y
2. Il valore iniziale di GridCell per il cube e' da rivedere
3. La somma di GridCoor con GridStep andrebbe gestita meglio
  - avvengono 2 conversioni da usize a f32
