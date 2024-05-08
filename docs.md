# robot functions

- `robot.move(number: pixels)`
- `robot.turn(number: degrees)`
- `robot.shoot()`
- `robot.raycast` = `"ship"`, `"bullet"`, `"rock"`, `"none"`, `"wall"`
- `robot.raycast_dist` = number in pixels
- `robot.x`
- `robot.y`
- `robot.rayhit_x`
- `robot.rayhit_y`

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
