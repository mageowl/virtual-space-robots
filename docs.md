# robot functions

- `robot_api.move(number: pixels)`
- `robot_api.turn(number: degrees)`
- `robot_api.shoot()`
- `robot_api.raycast` = `"ship"`, `"bullet"`, `"rock"`, `"none"`, `"wall"`
- `robot_api.raycast_dist` = number in pixels
- `robot_api.x`
- `robot_api.y`
- `robot_api.rayhit_x`
- `robot_api.rayhit_y`
- `robot_api.rotation`

# basics of bean script

to make a function

```beanscript
fn(<my_function>): {
    // code...
}
```

variables

```beanscript
let(<my_variable_name>): // value
```

loops

```beanscript
while: {
    if(/* condition */): return(false) // <- breaks while
    // code...
}

repeat(n): {
    // code...
}
```

lists

```beanscript
let(<array>): list(1, 2, 3)
array.for(<item>): {
    print(item)
}
```
