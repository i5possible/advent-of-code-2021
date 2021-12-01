const { readData } = require("./readfile");
const input = readData("../input/day1");
const lines = input.split(/\n/);

const question1 = (lines) => {
  const count = lines
    .map((line) => parseInt(line))
    .map((data, index, array) =>
      index === 0 ? 0 : data > array[index - 1] ? 1 : 0
    )
    .reduce((acc, cur) => acc + cur, 0);

  console.log(count);
};

const question2 = (lines) => {
  const count = lines
    .map((line) => parseInt(line))
    .map((data, index, array) =>
      index < 2 ? Infinity : data + array[index - 2] + array[index - 1]
    )
    .map((data, index, array) =>
      index === 0 ? 0 : data > array[index - 1] ? 1 : 0
    )
    .reduce((acc, cur) => acc + cur, 0);
  console.log(count);
};

question2(lines);
