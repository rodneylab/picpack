import { image_placeholder, input_image_hash } from "@rodneylab/picpack";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { assert, expect, test } from "vitest";

test("it generates expected output", () => {
  // prepare
  const __dirname = resolve();
  const imagePath = join(__dirname, "./images/field.jpg");
  const imageBytes = readFileSync(imagePath);

  // act
  const { average, base64, metadata, error } = image_placeholder(imageBytes);

  // assert
  assert.typeOf(image_placeholder, "function");

  expect(average).toBe("#6a7774ff");
  expect(base64).toBe(
    "data:image/jpeg;base64,/9j/4AAQSkZJRgABAgAAAQABAAD/wAARCAAXACADAREAAhEBAxEB/9sAQwADAgIDAgIDAwMDBAMDBAUIBQUEBAUKBwcGCAwKDAwLCgsLDQ4SEA0OEQ4LCxAWEBETFBUVFQwPFxgWFBgSFBUU/9sAQwEDBAQFBAUJBQUJFA0LDRQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQUFBQU/8QAHwAAAQUBAQEBAQEAAAAAAAAAAAECAwQFBgcICQoL/8QAtRAAAgEDAwIEAwUFBAQAAAF9AQIDAAQRBRIhMUEGE1FhByJxFDKBkaEII0KxwRVS0fAkM2JyggkKFhcYGRolJicoKSo0NTY3ODk6Q0RFRkdISUpTVFVWV1hZWmNkZWZnaGlqc3R1dnd4eXqDhIWGh4iJipKTlJWWl5iZmqKjpKWmp6ipqrKztLW2t7i5usLDxMXGx8jJytLT1NXW19jZ2uHi4+Tl5ufo6erx8vP09fb3+Pn6/8QAHwEAAwEBAQEBAQEBAQAAAAAAAAECAwQFBgcICQoL/8QAtREAAgECBAQDBAcFBAQAAQJ3AAECAxEEBSExBhJBUQdhcRMiMoEIFEKRobHBCSMzUvAVYnLRChYkNOEl8RcYGRomJygpKjU2Nzg5OkNERUZHSElKU1RVVldYWVpjZGVmZ2hpanN0dXZ3eHl6goOEhYaHiImKkpOUlZaXmJmaoqOkpaanqKmqsrO0tba3uLm6wsPExcbHyMnK0tPU1dbX2Nna4uPk5ebn6Onq8vP09fb3+Pn6/9oADAMBAAIRAxEAPwD1yD4f25XIcfnX0iPJZJN4Sjt4GAYVqiTm7DwrnVcg96GgNrxZoohslGR0rNoaPmbw/wDtW3FwvLn868FY5dzXlOhX9pNp1O563WOQuUpQ/tMQ2d5lnGc1f11dw5Sl4y/aointwFk/WoeNiHKeX6J8LIYU4YV+ZLFyO2xqj4fIiN89dEcXInlPO/E/g8w32Fkxz61r9akLlH2/w1GpW4LyfrSeKkPlP//Z",
  );
  assert.typeOf(error, "undefined");

  const { width, height, format } = metadata;
  expect(width).toBe(100);
  expect(height).toBe(75);
  expect(format).toBe("image/jpeg");
});

test("it generates expected input image hash", () => {
  // prepare
  const __dirname = resolve();
  const imagePath = join(__dirname, "./images/field.jpg");
  const imageBytes = readFileSync(imagePath);

  // act
  const result = input_image_hash(imageBytes);

  // assert
  assert.typeOf(input_image_hash, "function");

  assert.typeOf(result, "string");
  expect(result).toBe("f175a364a5b9");
});
