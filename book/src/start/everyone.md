# Xvc for Everyone

_Xvc Getting Started pages are written as stories and dialogues between tortoise (🐢) and hare (🐇)._

🐇 Hello tortoise. How are you? Let's take a selfie. Do you take selfies? I have lots of them. Terabytes of them. 

🐢 I don't have much selfies, you know. I don't change quickly and scenery is changing less often.  

🐇 I see. I have terabytes of them, but can't find a good solution to store them. How do you store your documents? I know you have documents, lots and lots of them. 

🐢 I track them with Git to track my evolving thoughts on text files. Images are different. I think it's not a good idea to keep images on Git, but there is a tool for that. 

🐇 What kind of tool? Not Git, but something different?

🐢 It's called Xvc. You can keep track of your selfies with it. You can backup them, and get them as needed. 

🐇 Tell me more about it. I have a directory in my home, `~/Selfies` and I have thousands of them. How will I start?

🐢 Xvc can be used as a standalone tool but better when used with Git. You can just type 

```shell
$ git init
$ xvc init
```

to start working with Xvc. 

🐇 It looks easy but I heard that Git is complicated. Will I need to learn it?

🐢 Ah,  no. If you're not willing to learn Git, you can just let Xvc to handle that. By default, it handles all Git operations about the changes it makes. If you want to push your files with someone, you may need to learn how to manage a repository. 

🐇 How do I track my files? 

🐢 You use `xvc file track` command. Do you have directories in `~/Selfies`?

🐇 Yep. I have. Lots of them.

🐢 Do you want to track all of them?

🐇 Almost all. Some of them are so private that I want to hide even from Xvc. 

🐢 You can use `.xvcignore` file to list them. Xvc ignores the files you list in `.xvcignore`. 

🐇 How do I add others? Could you give an example?

🐢 If you have a folder for today's selfies, type this in `~/Selfies`

```shell
$ xvc file track today/
```

and Xvc will track everything in that directory. 

🐇 Oh, that's easy. If I want to track everything not ignored, I can type `xvc file track` then.

🐢 You're a quick learner. 

_After some brief period 🐇 went to home and added files._

🐇 Now, I want to learn how to share my selfies. 

🐢 Xvc can store file contents in another location. First you must setup a _storage._ Do you use AWS S3?

🐇 Yes. I have buckets there. I want to keep my selfies in my `rabbit-hole`. 

🐢 You can configure Xvc to use it with `xvc storage new s3` command. You'll specify the region and bucket, and Xvc will prepare it. 

🐇 _types_

```shell
$ xvc storage new s3 --name selfies --region eu-lepus-1 --bucket rabbit-hole 
```

🐢 Now, you can send your files there with `xvc file send --to selfies`.

🐇 Is that all?

🐢 You will also need to push your Git files to another place. Do you have a Github account?

🐇 Ah, yeah, I have. 

🐢 Now create a repository for your selfies. We will configure Git to use it as `origin`. 

```shell
$ git remote add origin https://github.com/🐇/selfies
$ git push --set-upstream origin main
```

Now, you can share your selfies with your friends. 

🐇 Cool, but how Xvc knows my AWS password? Does it share my passwords?

🐢 No, never. You must allow your friends to read that bucket of yours. Xvc reads the credentials from AWS configuration, either from the file or the environment variables. 

🐇 How will they get my files?

🐢 First, they must clone the repository.

```shell
$ git clone https://github.com/🐇/selfies 
```

Then, they can get all files with:

```shell
$ cd selfies
$ xvc file get .
```

🐇 Oh, cool, they don't have to `xvc init` again? Right?

🐢 No, they don't. Xvc should be initialized only once per repository. When you have new selfies, you can share them with: 

```shell
$ xvc file track 
$ git push 
```

and your friends can receive the changes with

```shell
$ git pull 
$ xvc file get
```

🐇 The order of these commands are important, it looks. 

🐢 Yep. You add to Xvc first. Xvc automatically commits the changes to Git. Then you push Git changes to remote. Your friends first pull these changes, then get the actual files. 

🐇 Thank you tortoise. Let me get back to my hole. 

