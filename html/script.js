import init, {validate_header, get_polyomino, get_orders} from "./pkg/polyominos.js";

const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('2d');

const fetchBinaryData = (location) => {
  const xhr = new XMLHttpRequest();
  xhr.responseType = 'arraybuffer';
  return new Promise(function(resolve, reject) {
      xhr.onreadystatechange = function() {
          if (xhr.readyState === 4) {
              if (xhr.status >= 300) {
                  reject(new Error('Error, status code = ' + xhr.status));
              } else {
                  resolve(xhr.response);
              }
          }
      };
      xhr.open('get', location, true);
      xhr.send();
  });
};

init().then(async () => {
    const data = new Uint8Array(await fetchBinaryData("./twelve.pif"));
    if (!validate_header(data)) {
        window.alert("Error");
    }
    const order_map = get_orders(data);

    document.getElementById("select-button").addEventListener("click", (e) => {
        const order = parseInt(document.getElementById("order-select").value);
        const max = order_map[order][1];
        const ind = Math.floor(Math.random() * max);
        const polyomino = get_polyomino(data, order, ind, order_map);
        const max_x = Math.max(...polyomino.map((x) => x[0]));
        const max_y = Math.max(...polyomino.map((x) => x[1]));
        const width = canvas.width;
        const height = canvas.height;
        const size = Math.min((Math.min(width, height) - 50) / polyomino.length, 50);

        const x_orig = (width - size*(max_x+1)) / 2;
        const y_orig = (height - size*(max_y+1)) / 2;

        const BORDER = 1;

        ctx.clearRect(0, 0, width, height);
        ctx.fillStyle = 'black';
        for (const mino of polyomino) {
            ctx.fillRect(
                mino[0]*size + BORDER + x_orig, 
                mino[1]*size + BORDER + y_orig, 
                size-2*BORDER, size-2*BORDER);
        }

    })
});
