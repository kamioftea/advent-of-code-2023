const MarkdownIt = require('markdown-it')
const anchor = require('markdown-it-anchor')
const defList = require('markdown-it-deflist');
const highlight = require('./highlight')
const mathjax = require('markdown-it-mathjax')


const defaultOpts = {
    baseOpts:          {},
    headingPermalinks: true,
}

module.exports = (options = defaultOpts) => {
    const opts = {
        breaks:      false,
        highlight,
        html:        true,
        linkify:     false,
        typographer: true,

        ...(options.baseOpts)
    }


    return new MarkdownIt(opts)
        .use(anchor, {
            permalink: options.headingPermalinks ?
                           anchor.permalink.headerLink({
                               class:           'app-link--heading',
                               safariReaderFix: true
                           }) :
                           false
        })
        .use(defList)
        .use(mathjax());
}
