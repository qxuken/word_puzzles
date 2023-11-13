function onlyLetters(value) {
  return value.replaceAll(/[^a-zA-Z]+/g, '');
}

function compressLetterList(value) {
  return value.replaceAll(/-\d[ |\n]/g, ', ').replaceAll(/-\d/g, '');
}
