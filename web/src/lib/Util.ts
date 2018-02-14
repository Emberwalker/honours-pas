import hljs from "highlight.js";
import * as marked from "marked";
import Mutations from "./Mutations";

export function highlightingInit() {
  marked.setOptions({
    highlight(code, lang) {
      if (lang) {
        return hljs.highlight(lang, code, true).value;
      }
      return hljs.highlightAuto(code).value;
    },
    sanitize: true,
  });
}

export function parseMarkdown(md: string): string {
  return marked(md);
}

export function renderCodeBlock(el: any) {
  hljs.highlightBlock(el);
}
