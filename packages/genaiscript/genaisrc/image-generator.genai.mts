console.log("Generating image")

def("USER_INPUT", env.vars.user_input);


const inputs = {
    host: JSON.parse(env.vars.user_input).host,
    imageId: JSON.parse(env.vars.user_input).imageId
}

console.log(`![Generated Image](/generated/image/${inputs.imageId})`);
