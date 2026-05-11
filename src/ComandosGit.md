Cambiar de rama 
git checkout rama
git switch rama

Crear y cambiar
git checkout -b nueva
git switch -c nueva

Volver a la rama anterior
git checkout -
git switch -

Restaurar un archivo
git checkout archivo.js
git restore archivo.js*

borrar ramas en el servidor
git push origin --delete nombre-de-la-rama


git branch -d nombre-de-la-rama
git branch -D nombre-de-la-rama


borrar ramas locales que no exiten en el servidor
git fetch --prune
