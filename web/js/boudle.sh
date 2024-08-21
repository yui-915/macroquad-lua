minify \
  --bundle \
  --js-keep-var-names \
  --js-precision 0 \
  --output mq-glue.js \
  src/gl.js \
  src/audio.js \
  src/sapp_jsutils.js \
  src/quad-net.js \
  src/emscripten.js
echo ";" >>mq-glue.js
