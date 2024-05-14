<p align="center">
  <h1 align="center">Snaek</h1>
  <p align="center">
    <img height="400" src="https://fs.speykious.dev/snaek/snaek-preview.png" alt="Snake game preview">
  </p>
  <div align="center">My overengineered snake game!</div>
</p>

&nbsp;

> Art mostly by @jumbledFox.
> Check his [minesweeper](https://github.com/jumbledFox/minesweeper) made in the same style!

## Why did I make this?

It was actually just an excuse for me to make my own UI framework from scratch.

I had recently been reading the first free articles of [Ryan Fleury's UI series](https://www.rfleury.com/p/ui-series-table-of-contents) and found it quite fascinating. Before reading this I thought immediate-mode libraries were incapable of handling complex layouts among other things, but it's simply not the case. You can actually perfectly integrate declarative layouts, animations, and persistent widget state with an immediate-mode API and it's most definitely not as battery-draining as people make it to be. (Well, ignoring this project, since I didn't make any rendering optimizations at all...)

So now I really want to use these techniques in any potential future GUI project I might have. That being said, doing stuff with pixels directly has somewhat simplified the abstractions I needed to create (compared to a graphics API like OpenGL or Vulkan), not to mention that I didn't implement anything like a scroll view or a drop down, so it'll probably take way more time to make something that flexible.

## Highlights

While coding the renderer for this game, I created the prettiest bug I've ever seen:

<p align="center">
  <img height="400" src="https://fs.speykious.dev/snaek/prettiest-bug-i-ever-made.png" alt="Prettiest bug I ever made">
</p>

You can run the code that produced this bug by checking out [this commit](https://github.com/Speykious/snaek/commit/9e1bbe9d9b0187037d5ec48ca6dd1bc28b1f4f97) if you're curious.

## License

This project is licensed under [MIT](/LICENSE).