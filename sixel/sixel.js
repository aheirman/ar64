/*
  COPYRIGHT ARON HEIRMAN 2023 
  ALL RIGHTS RESERVED
*/

import puppeteer from 'puppeteer';
import * as readline from 'node:readline/promises';
import { stdin as input, stdout as output } from 'node:process';
import process from 'node:process';

import { spawn, spawnSync } from 'node:child_process';

(async () => {
  const browser = await puppeteer.launch(
    {
      headless: true,
      defaultViewport: {
        width:1050,
        height:800
      }
  });
  const page = await browser.newPage();

  await page.goto('http://localhost:5173/');

  const rl = readline.createInterface({ input, output });
  let loop = true;
  while (loop) {
    await page.waitForTimeout(100)
    await page.screenshot({
      type: "png",
      path: "/tmp/img.png",
      omitBackground: true,
    });
    let result = spawnSync('img2sixel', ['/tmp/img.png'])
    console.log(result["stdout"].toString())

    const answer = await rl.question('COMMAND: ');
    if (answer == "q") {
      loop = false;
      process.exit(1);
    } else if (answer == "n") {
      await page.click('text/step')
    } else if (answer == "l") {
      await page.click('text/load')
    } else if (answer == "help") {
      console.log("a64-svixel")
      console.log("[qnlh] quit | next | load | help")
    } else {
      console.log("UNKNOWN COMMAND!")
    }

  }
  rl.close();

  await browser.close();
})();


