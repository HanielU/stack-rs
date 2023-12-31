// my-preset.ts
import { Preset } from "unocss";
import { handler as h, variantGetParameter } from "@unocss/preset-mini/utils";

export const myPreset: Preset = {
  name: "my-preset",

  shortcuts: [
    [
      // flex-s stands for flex-shortcut
      // to avoid mixups with default flex utilities like flex-wrap
      /^flex-s-(start|center|between|evenly|around|end)(-(start|center|baseline|end))?$/,
      ([, justify, align]) => `flex justify-${justify} items${align || "-center"}`,
      { layer: "default" },
    ],
    // use when width and height values are the same
    [/^square-(.*)$/, ([, v]) => `h-${v} w-${v}`, { layer: "utilities" }],
    [
      /^br(-\w+)?$/, // h - hyphen | v - value
      ([, hAndV], { theme }) => {
        const [, v] = hAndV?.split("-") || [];
        return v ? `rounded-${theme[v] || v}` : "rounded";
      },
      { layer: "default" },
    ],
    [
      /^scrollbar-f-(thin)-(.*)$/,
      ([, size, colors]) => `[scrollbar-width:${size}] [scrollbar-color:${colors}]`,
      { layer: "utilities" },
    ],
  ],

  variants: [
    {
      // adds support for "@min-[width]:class" and "@min-h-[width]:class"
      // or
      // "@min-width:class" and "@min-h-width:class"
      name: "arbitrary-media-query",
      match(matcher, { theme }) {
        // prefix with @ to specify that it's a media query
        const minVariant = variantGetParameter("@min-", matcher, [":"]);
        const maxVariant = variantGetParameter("@max-", matcher, [":"]);
        const minHeightVariant = variantGetParameter("@min-h-", matcher, [":"]);
        const maxHeightVariant = variantGetParameter("@max-h-", matcher, [":"]);

        // the order that we check the variants is important
        // because we want to match the most specific one
        const matched =
          (minHeightVariant && {
            type: "min-h",
            variant: minHeightVariant,
          }) ||
          (maxHeightVariant && {
            type: "max-h",
            variant: maxHeightVariant,
          }) ||
          (minVariant && {
            type: "min",
            variant: minVariant,
          }) ||
          (maxVariant && {
            type: "max",
            variant: maxVariant,
          });

        if (matched?.variant) {
          const [match, rest] = matched.variant;
          // this is for extracting the value from the match and
          // makes sure it either has no brackets or has brackets
          const extractedValue =
            h.bracket(match) || (!match.startsWith("[") && !match.endsWith("]") && match) || "";
          const endsWithUnit = /^\d+(em|px|rem)$/.test(extractedValue);
          const isOnlyNum = /^\d+$/.test(extractedValue);

          if (endsWithUnit || isOnlyNum || theme["breakpoints"][extractedValue]) {
            return {
              matcher: rest,
              layer: "utilities",
              handle: (input, next) =>
                next({
                  ...input,
                  parent: `${input.parent ? `${input.parent} $$ ` : ""}@media (${
                    matched.type == "min"
                      ? "min-width"
                      : matched.type == "max"
                      ? "max-width"
                      : matched.type == "min-h"
                      ? "min-height"
                      : "max-height"
                  }:${
                    endsWithUnit
                      ? extractedValue
                      : isOnlyNum
                      ? extractedValue + "px"
                      : theme["breakpoints"][extractedValue]
                  })`,
                }),
            };
          }
        }
      },
    },
    {
      name: "firefox-only",
      match(matcher) {
        const ffVariant = variantGetParameter("@ff", matcher, [":"]);
        if (ffVariant) {
          const [, rest] = ffVariant;
          return {
            matcher: rest,
            handle: (input, next) =>
              next({
                ...input,
                parent: `${input.parent ? `${input.parent} $$ ` : ""}@-moz-document url-prefix()`,
              }),
          };
        }
      },
    },
    matcher => {
      const [m1, m2, m3] = ["scrollbar:", "scrollbar-track:", "scrollbar-thumb:"];
      let matchedStr = "";

      if (matcher.startsWith(m1)) {
        matchedStr = m1;
      } else if (matcher.startsWith(m2)) {
        matchedStr = m2;
      } else if (matcher.startsWith(m3)) {
        matchedStr = m3;
      } else {
        return matcher;
      }

      return {
        matcher: matcher.slice(matchedStr.length),
        selector: s =>
          `${s}::-webkit-scrollbar${
            matchedStr == m2 ? "-track" : matchedStr == m3 ? "-thumb" : ""
          }`,
        layer: "default",
      };
    },
  ],
};
