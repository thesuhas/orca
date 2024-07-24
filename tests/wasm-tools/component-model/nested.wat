(component
  (import "i1" (core module))

  (core module)
  (core module)

  (core module (export "x"))

  (component
    (core module)
  )

  (component
    (core module $m)
    (import "a" (func (param "p" string)))
    (export "b" (core module $m))
  )
)