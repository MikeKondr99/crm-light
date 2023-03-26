import eel

eel.init("front")

@eel.expose
def call_in_js(x):
    print(x)

eel.call_in_py("hello from js")

eel.start("template/auth.html", mode='chrome-app')