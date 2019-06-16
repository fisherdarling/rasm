(module
  (type $t0 (func (param i32)))
  (type $t1 (func (param i32 i32 i32) (result i32)))
  (func $dlmalloc::dlmalloc::Dlmalloc::unlink_large_chunk::hf712b91716024651 (type $t0) (param $p0 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32)
    get_local $p0
    i32.load offset=24
    set_local $l0
    block $B0
      block $B1
        block $B2
          block $B3
            get_local $p0
            i32.load offset=12
            tee_local $l1
            get_local $p0
            i32.eq
            br_if $B3
            get_local $p0
            i32.load offset=8
            tee_local $l2
            get_local $l1
            i32.store offset=12
            get_local $l1
            get_local $l2
            i32.store offset=8
            get_local $l0
            br_if $B2
            br $B1
          end
          block $B4
            get_local $p0
            i32.const 20
            i32.add
            tee_local $l2
            get_local $p0
            i32.const 16
            i32.add
            get_local $l2
            i32.load
            select
            tee_local $l3
            i32.load
            tee_local $l2
            i32.eqz
            br_if $B4
            block $B5
              loop $L6
                get_local $l3
                set_local $l4
                block $B7
                  get_local $l2
                  tee_local $l1
                  i32.const 20
                  i32.add
                  tee_local $l3
                  i32.load
                  tee_local $l2
                  i32.eqz
                  br_if $B7
                  get_local $l2
                  br_if $L6
                  br $B5
                end
                get_local $l1
                i32.const 16
                i32.add
                set_local $l3
                get_local $l1
                i32.load offset=16
                tee_local $l2
                br_if $L6
              end
            end
            get_local $l4
            i32.const 0
            i32.store
            get_local $l0
            br_if $B2
            br $B1
          end
          i32.const 0
          set_local $l1
          get_local $l0
          i32.eqz
          br_if $B1
        end
        block $B8
          block $B9
            get_local $p0
            i32.load offset=28
            tee_local $l3
            i32.const 2
            i32.shl
            i32.const 1296
            i32.add
            tee_local $l2
            i32.load
            get_local $p0
            i32.eq
            br_if $B9
            get_local $l0
            i32.const 16
            i32.add
            get_local $l0
            i32.const 20
            i32.add
            get_local $l0
            i32.load offset=16
            get_local $p0
            i32.eq
            select
            get_local $l1
            i32.store
            get_local $l1
            br_if $B8
            br $B1
          end
          get_local $l2
          get_local $l1
          i32.store
          get_local $l1
          i32.eqz
          br_if $B0
        end
        get_local $l1
        get_local $l0
        i32.store offset=24
        block $B10
          get_local $p0
          i32.load offset=16
          tee_local $l2
          i32.eqz
          br_if $B10
          get_local $l1
          get_local $l2
          i32.store offset=16
          get_local $l2
          get_local $l1
          i32.store offset=24
        end
        get_local $p0
        i32.const 20
        i32.add
        i32.load
        tee_local $l2
        i32.eqz
        br_if $B1
        get_local $l1
        i32.const 20
        i32.add
        get_local $l2
        i32.store
        get_local $l2
        get_local $l1
        i32.store offset=24
      end
      return
    end
    i32.const 0
    i32.const 0
    i32.load offset=1028
    i32.const -2
    get_local $l3
    i32.rotl
    i32.and
    i32.store offset=1028)
  (func $sum_of_squares (export "sum_of_squares") (type $t1) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32)
    block $B0
      block $B1
        block $B2
          block $B3
            get_local $p2
            i32.eqz
            br_if $B3
            get_local $p2
            i32.const 2
            i32.shl
            set_local $l0
            i32.const 0
            set_local $l1
            get_local $p0
            set_local $p2
            loop $L4
              get_local $p2
              i32.load
              tee_local $l2
              get_local $l2
              i32.mul
              get_local $l1
              i32.add
              set_local $l1
              get_local $p2
              i32.const 4
              i32.add
              set_local $p2
              get_local $l0
              i32.const -4
              i32.add
              tee_local $l0
              br_if $L4
            end
            get_local $p1
            br_if $B2
            br $B1
          end
          i32.const 0
          set_local $l1
          get_local $p1
          i32.eqz
          br_if $B1
        end
        get_local $p0
        i32.const -8
        i32.add
        tee_local $p2
        get_local $p0
        i32.const -4
        i32.add
        i32.load
        tee_local $p0
        i32.const -8
        i32.and
        tee_local $l0
        i32.add
        set_local $l2
        block $B5
          get_local $p0
          i32.const 1
          i32.and
          br_if $B5
          get_local $p0
          i32.const 3
          i32.and
          i32.eqz
          br_if $B1
          get_local $p2
          i32.load
          tee_local $p0
          get_local $l0
          i32.add
          set_local $l0
          block $B6
            block $B7
              block $B8
                i32.const 0
                i32.load offset=1432
                get_local $p2
                get_local $p0
                i32.sub
                tee_local $p2
                i32.eq
                br_if $B8
                get_local $p0
                i32.const 255
                i32.gt_u
                br_if $B7
                get_local $p2
                i32.load offset=12
                tee_local $p1
                get_local $p2
                i32.load offset=8
                tee_local $l3
                i32.eq
                br_if $B6
                get_local $l3
                get_local $p1
                i32.store offset=12
                get_local $p1
                get_local $l3
                i32.store offset=8
                br $B5
              end
              get_local $l2
              i32.load offset=4
              tee_local $p0
              i32.const 3
              i32.and
              i32.const 3
              i32.ne
              br_if $B5
              i32.const 0
              get_local $l0
              i32.store offset=1424
              get_local $l2
              i32.const 4
              i32.add
              get_local $p0
              i32.const -2
              i32.and
              i32.store
              get_local $p2
              get_local $l0
              i32.const 1
              i32.or
              i32.store offset=4
              get_local $l2
              get_local $l0
              i32.store
              get_local $l1
              return
            end
            get_local $p2
            call $dlmalloc::dlmalloc::Dlmalloc::unlink_large_chunk::hf712b91716024651
            br $B5
          end
          i32.const 0
          i32.const 0
          i32.load offset=1024
          i32.const -2
          get_local $p0
          i32.const 3
          i32.shr_u
          i32.rotl
          i32.and
          i32.store offset=1024
        end
        block $B9
          block $B10
            block $B11
              block $B12
                block $B13
                  block $B14
                    block $B15
                      block $B16
                        block $B17
                          get_local $l2
                          i32.load offset=4
                          tee_local $p0
                          i32.const 2
                          i32.and
                          br_if $B17
                          i32.const 0
                          i32.load offset=1436
                          get_local $l2
                          i32.eq
                          br_if $B16
                          i32.const 0
                          i32.load offset=1432
                          get_local $l2
                          i32.eq
                          br_if $B15
                          get_local $p0
                          i32.const -8
                          i32.and
                          tee_local $p1
                          get_local $l0
                          i32.add
                          set_local $l0
                          get_local $p1
                          i32.const 255
                          i32.gt_u
                          br_if $B14
                          get_local $l2
                          i32.load offset=12
                          tee_local $p1
                          get_local $l2
                          i32.load offset=8
                          tee_local $l2
                          i32.eq
                          br_if $B13
                          get_local $l2
                          get_local $p1
                          i32.store offset=12
                          get_local $p1
                          get_local $l2
                          i32.store offset=8
                          br $B12
                        end
                        get_local $l2
                        i32.const 4
                        i32.add
                        get_local $p0
                        i32.const -2
                        i32.and
                        i32.store
                        get_local $p2
                        get_local $l0
                        i32.const 1
                        i32.or
                        i32.store offset=4
                        get_local $p2
                        get_local $l0
                        i32.add
                        get_local $l0
                        i32.store
                        br $B9
                      end
                      i32.const 0
                      get_local $p2
                      i32.store offset=1436
                      i32.const 0
                      i32.const 0
                      i32.load offset=1428
                      get_local $l0
                      i32.add
                      tee_local $l0
                      i32.store offset=1428
                      get_local $p2
                      get_local $l0
                      i32.const 1
                      i32.or
                      i32.store offset=4
                      block $B18
                        get_local $p2
                        i32.const 0
                        i32.load offset=1432
                        i32.ne
                        br_if $B18
                        i32.const 0
                        i32.const 0
                        i32.store offset=1424
                        i32.const 0
                        i32.const 0
                        i32.store offset=1432
                      end
                      i32.const 0
                      i32.load offset=1464
                      get_local $l0
                      i32.ge_u
                      br_if $B1
                      block $B19
                        get_local $l0
                        i32.const 41
                        i32.lt_u
                        br_if $B19
                        i32.const 1448
                        set_local $l0
                        loop $L20
                          block $B21
                            get_local $l0
                            i32.load
                            tee_local $l2
                            get_local $p2
                            i32.gt_u
                            br_if $B21
                            get_local $l2
                            get_local $l0
                            i32.load offset=4
                            i32.add
                            get_local $p2
                            i32.gt_u
                            br_if $B19
                          end
                          get_local $l0
                          i32.load offset=8
                          tee_local $l0
                          br_if $L20
                        end
                      end
                      i32.const 0
                      set_local $p2
                      i32.const 0
                      i32.load offset=1456
                      tee_local $l0
                      i32.eqz
                      br_if $B11
                      loop $L22
                        get_local $p2
                        i32.const 1
                        i32.add
                        set_local $p2
                        get_local $l0
                        i32.load offset=8
                        tee_local $l0
                        br_if $L22
                      end
                      get_local $p2
                      i32.const 4095
                      get_local $p2
                      i32.const 4095
                      i32.gt_u
                      select
                      set_local $p2
                      br $B10
                    end
                    i32.const 0
                    get_local $p2
                    i32.store offset=1432
                    i32.const 0
                    i32.const 0
                    i32.load offset=1424
                    get_local $l0
                    i32.add
                    tee_local $l0
                    i32.store offset=1424
                    get_local $p2
                    get_local $l0
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    get_local $p2
                    get_local $l0
                    i32.add
                    get_local $l0
                    i32.store
                    get_local $l1
                    return
                  end
                  get_local $l2
                  call $dlmalloc::dlmalloc::Dlmalloc::unlink_large_chunk::hf712b91716024651
                  br $B12
                end
                i32.const 0
                i32.const 0
                i32.load offset=1024
                i32.const -2
                get_local $p0
                i32.const 3
                i32.shr_u
                i32.rotl
                i32.and
                i32.store offset=1024
              end
              get_local $p2
              get_local $l0
              i32.const 1
              i32.or
              i32.store offset=4
              get_local $p2
              get_local $l0
              i32.add
              get_local $l0
              i32.store
              get_local $p2
              i32.const 0
              i32.load offset=1432
              i32.ne
              br_if $B9
              i32.const 0
              get_local $l0
              i32.store offset=1424
              get_local $l1
              return
            end
            i32.const 4095
            set_local $p2
          end
          i32.const 0
          i32.const -1
          i32.store offset=1464
          i32.const 0
          get_local $p2
          i32.store offset=1472
          get_local $l1
          return
        end
        block $B23
          block $B24
            block $B25
              block $B26
                block $B27
                  block $B28
                    block $B29
                      block $B30
                        get_local $l0
                        i32.const 255
                        i32.gt_u
                        br_if $B30
                        get_local $l0
                        i32.const 3
                        i32.shr_u
                        tee_local $l2
                        i32.const 3
                        i32.shl
                        i32.const 1032
                        i32.add
                        set_local $l0
                        i32.const 0
                        i32.load offset=1024
                        tee_local $p0
                        i32.const 1
                        get_local $l2
                        i32.const 31
                        i32.and
                        i32.shl
                        tee_local $l2
                        i32.and
                        i32.eqz
                        br_if $B29
                        get_local $l0
                        i32.const 8
                        i32.add
                        set_local $p0
                        get_local $l0
                        i32.load offset=8
                        set_local $l2
                        br $B28
                      end
                      i32.const 0
                      set_local $l2
                      block $B31
                        get_local $l0
                        i32.const 8
                        i32.shr_u
                        tee_local $p0
                        i32.eqz
                        br_if $B31
                        i32.const 31
                        set_local $l2
                        get_local $l0
                        i32.const 16777215
                        i32.gt_u
                        br_if $B31
                        get_local $l0
                        i32.const 38
                        get_local $p0
                        i32.clz
                        tee_local $l2
                        i32.sub
                        i32.const 31
                        i32.and
                        i32.shr_u
                        i32.const 1
                        i32.and
                        i32.const 31
                        get_local $l2
                        i32.sub
                        i32.const 1
                        i32.shl
                        i32.or
                        set_local $l2
                      end
                      get_local $p2
                      i64.const 0
                      i64.store offset=16 align=4
                      get_local $p2
                      i32.const 28
                      i32.add
                      get_local $l2
                      i32.store
                      get_local $l2
                      i32.const 2
                      i32.shl
                      i32.const 1296
                      i32.add
                      set_local $p0
                      i32.const 0
                      i32.load offset=1028
                      tee_local $p1
                      i32.const 1
                      get_local $l2
                      i32.const 31
                      i32.and
                      i32.shl
                      tee_local $l3
                      i32.and
                      i32.eqz
                      br_if $B27
                      get_local $p0
                      i32.load
                      tee_local $p1
                      i32.load offset=4
                      i32.const -8
                      i32.and
                      get_local $l0
                      i32.ne
                      br_if $B26
                      get_local $p1
                      set_local $l2
                      br $B25
                    end
                    i32.const 0
                    get_local $p0
                    get_local $l2
                    i32.or
                    i32.store offset=1024
                    get_local $l0
                    i32.const 8
                    i32.add
                    set_local $p0
                    get_local $l0
                    set_local $l2
                  end
                  get_local $p0
                  get_local $p2
                  i32.store
                  get_local $l2
                  get_local $p2
                  i32.store offset=12
                  get_local $p2
                  get_local $l0
                  i32.store offset=12
                  get_local $p2
                  get_local $l2
                  i32.store offset=8
                  get_local $l1
                  return
                end
                get_local $p0
                get_local $p2
                i32.store
                i32.const 0
                get_local $p1
                get_local $l3
                i32.or
                i32.store offset=1028
                get_local $p2
                i32.const 24
                i32.add
                get_local $p0
                i32.store
                get_local $p2
                get_local $p2
                i32.store offset=8
                get_local $p2
                get_local $p2
                i32.store offset=12
                br $B23
              end
              get_local $l0
              i32.const 0
              i32.const 25
              get_local $l2
              i32.const 1
              i32.shr_u
              i32.sub
              i32.const 31
              i32.and
              get_local $l2
              i32.const 31
              i32.eq
              select
              i32.shl
              set_local $p0
              loop $L32
                get_local $p1
                get_local $p0
                i32.const 29
                i32.shr_u
                i32.const 4
                i32.and
                i32.add
                i32.const 16
                i32.add
                tee_local $l3
                i32.load
                tee_local $l2
                i32.eqz
                br_if $B24
                get_local $p0
                i32.const 1
                i32.shl
                set_local $p0
                get_local $l2
                set_local $p1
                get_local $l2
                i32.load offset=4
                i32.const -8
                i32.and
                get_local $l0
                i32.ne
                br_if $L32
              end
            end
            get_local $l2
            i32.load offset=8
            tee_local $l0
            get_local $p2
            i32.store offset=12
            get_local $l2
            get_local $p2
            i32.store offset=8
            get_local $p2
            get_local $l2
            i32.store offset=12
            get_local $p2
            get_local $l0
            i32.store offset=8
            get_local $p2
            i32.const 24
            i32.add
            i32.const 0
            i32.store
            br $B23
          end
          get_local $l3
          get_local $p2
          i32.store
          get_local $p2
          i32.const 24
          i32.add
          get_local $p1
          i32.store
          get_local $p2
          get_local $p2
          i32.store offset=12
          get_local $p2
          get_local $p2
          i32.store offset=8
        end
        i32.const 0
        i32.const 0
        i32.load offset=1472
        i32.const -1
        i32.add
        tee_local $p2
        i32.store offset=1472
        get_local $p2
        i32.eqz
        br_if $B0
      end
      get_local $l1
      return
    end
    block $B33
      block $B34
        i32.const 0
        i32.load offset=1456
        tee_local $l0
        i32.eqz
        br_if $B34
        i32.const 0
        set_local $p2
        loop $L35
          get_local $p2
          i32.const 1
          i32.add
          set_local $p2
          get_local $l0
          i32.load offset=8
          tee_local $l0
          br_if $L35
        end
        get_local $p2
        i32.const 4095
        get_local $p2
        i32.const 4095
        i32.gt_u
        select
        set_local $p2
        br $B33
      end
      i32.const 4095
      set_local $p2
    end
    i32.const 0
    get_local $p2
    i32.store offset=1472
    get_local $l1)
  (table $T0 1 1 anyfunc)
  (memory $memory (export "memory") 17)
  (data (i32.const 1024) "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"))

