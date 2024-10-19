To run the project in development we need to run the project in trunk and run the tailwind in watch mode
Follow this article to integrate tailwind https://dev.to/arctic_hen7/how-to-set-up-tailwind-css-with-yew-and-trunk-il9

You run the tailwind in watch mode only for production if you already created a tailwind.css file in local you don't have to

To create a local tailwind.css
```
npm i -g tailwindcss
tailwindcss -o ./tailwind.css
```



```sh
trunk serve
```

```sh
npx tailwindcss -c ./tailwind.config.js -o ./tailwind.css --watch
```


## How to Deploy to gh pages?

In order to deploy to gh pages, first create a release

```sh
trunk build --release --public-url /
```

Then copy the content of the /dist folder and add to the /docs folder