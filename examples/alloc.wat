(module
  (type $t0 (func (param i32 i32 i32) (result i32)))
  (type $t1 (func (param i32)))
  (type $t2 (func (param i32 i32)))
  (type $t3 (func (result i32)))
  (func $sum_vec (export "sum_vec") (type $t0) (param $p0 i32) (param $p1 i32) (param $p2 i32) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32)
    block $B0
      block $B1
        block $B2
          get_local $p2
          i32.eqz
          br_if $B2
          get_local $p2
          i32.const 2
          i32.shl
          set_local $l0
          i32.const 0
          set_local $l1
          get_local $p0
          set_local $p2
          loop $L3
            get_local $p2
            i32.load
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
            br_if $L3
          end
          get_local $p1
          br_if $B1
          br $B0
        end
        i32.const 0
        set_local $l1
        get_local $p1
        i32.eqz
        br_if $B0
      end
      get_local $p0
      i32.const -8
      i32.add
      tee_local $p2
      get_local $p0
      i32.const -4
      i32.add
      i32.load
      tee_local $p1
      i32.const -8
      i32.and
      tee_local $l0
      i32.add
      set_local $p0
      block $B4
        get_local $p1
        i32.const 1
        i32.and
        br_if $B4
        get_local $p1
        i32.const 3
        i32.and
        i32.eqz
        br_if $B0
        get_local $p2
        i32.load
        tee_local $p1
        get_local $l0
        i32.add
        set_local $l0
        block $B5
          block $B6
            block $B7
              i32.const 0
              i32.load offset=1440
              get_local $p2
              get_local $p1
              i32.sub
              tee_local $p2
              i32.eq
              br_if $B7
              get_local $p1
              i32.const 255
              i32.gt_u
              br_if $B6
              get_local $p2
              i32.load offset=12
              tee_local $l2
              get_local $p2
              i32.load offset=8
              tee_local $l3
              i32.eq
              br_if $B5
              get_local $l3
              get_local $l2
              i32.store offset=12
              get_local $l2
              get_local $l3
              i32.store offset=8
              br $B4
            end
            get_local $p0
            i32.load offset=4
            tee_local $p1
            i32.const 3
            i32.and
            i32.const 3
            i32.ne
            br_if $B4
            i32.const 0
            get_local $l0
            i32.store offset=1432
            get_local $p0
            i32.const 4
            i32.add
            get_local $p1
            i32.const -2
            i32.and
            i32.store
            get_local $p2
            get_local $l0
            i32.const 1
            i32.or
            i32.store offset=4
            get_local $p0
            get_local $l0
            i32.store
            get_local $l1
            return
          end
          get_local $p2
          call $dlmalloc::dlmalloc::Dlmalloc::unlink_large_chunk::hf712b91716024651
          br $B4
        end
        i32.const 0
        i32.const 0
        i32.load offset=1032
        i32.const -2
        get_local $p1
        i32.const 3
        i32.shr_u
        i32.rotl
        i32.and
        i32.store offset=1032
      end
      block $B8
        block $B9
          block $B10
            block $B11
              block $B12
                block $B13
                  block $B14
                    block $B15
                      block $B16
                        get_local $p0
                        i32.load offset=4
                        tee_local $p1
                        i32.const 2
                        i32.and
                        br_if $B16
                        i32.const 0
                        i32.load offset=1444
                        get_local $p0
                        i32.eq
                        br_if $B15
                        i32.const 0
                        i32.load offset=1440
                        get_local $p0
                        i32.eq
                        br_if $B14
                        get_local $p1
                        i32.const -8
                        i32.and
                        tee_local $l2
                        get_local $l0
                        i32.add
                        set_local $l0
                        get_local $l2
                        i32.const 255
                        i32.gt_u
                        br_if $B13
                        get_local $p0
                        i32.load offset=12
                        tee_local $l2
                        get_local $p0
                        i32.load offset=8
                        tee_local $p0
                        i32.eq
                        br_if $B12
                        get_local $p0
                        get_local $l2
                        i32.store offset=12
                        get_local $l2
                        get_local $p0
                        i32.store offset=8
                        br $B11
                      end
                      get_local $p0
                      i32.const 4
                      i32.add
                      get_local $p1
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
                      br $B8
                    end
                    i32.const 0
                    get_local $p2
                    i32.store offset=1444
                    i32.const 0
                    i32.const 0
                    i32.load offset=1436
                    get_local $l0
                    i32.add
                    tee_local $l0
                    i32.store offset=1436
                    get_local $p2
                    get_local $l0
                    i32.const 1
                    i32.or
                    i32.store offset=4
                    block $B17
                      get_local $p2
                      i32.const 0
                      i32.load offset=1440
                      i32.ne
                      br_if $B17
                      i32.const 0
                      i32.const 0
                      i32.store offset=1432
                      i32.const 0
                      i32.const 0
                      i32.store offset=1440
                    end
                    i32.const 0
                    i32.load offset=1472
                    get_local $l0
                    i32.ge_u
                    br_if $B0
                    block $B18
                      get_local $l0
                      i32.const 41
                      i32.lt_u
                      br_if $B18
                      i32.const 1456
                      set_local $l0
                      loop $L19
                        block $B20
                          get_local $l0
                          i32.load
                          tee_local $p0
                          get_local $p2
                          i32.gt_u
                          br_if $B20
                          get_local $p0
                          get_local $l0
                          i32.load offset=4
                          i32.add
                          get_local $p2
                          i32.gt_u
                          br_if $B18
                        end
                        get_local $l0
                        i32.load offset=8
                        tee_local $l0
                        br_if $L19
                      end
                    end
                    i32.const 0
                    set_local $p2
                    i32.const 0
                    i32.load offset=1464
                    tee_local $l0
                    i32.eqz
                    br_if $B10
                    loop $L21
                      get_local $p2
                      i32.const 1
                      i32.add
                      set_local $p2
                      get_local $l0
                      i32.load offset=8
                      tee_local $l0
                      br_if $L21
                    end
                    get_local $p2
                    i32.const 4095
                    get_local $p2
                    i32.const 4095
                    i32.gt_u
                    select
                    set_local $p2
                    br $B9
                  end
                  i32.const 0
                  get_local $p2
                  i32.store offset=1440
                  i32.const 0
                  i32.const 0
                  i32.load offset=1432
                  get_local $l0
                  i32.add
                  tee_local $l0
                  i32.store offset=1432
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
                get_local $p0
                call $dlmalloc::dlmalloc::Dlmalloc::unlink_large_chunk::hf712b91716024651
                br $B11
              end
              i32.const 0
              i32.const 0
              i32.load offset=1032
              i32.const -2
              get_local $p1
              i32.const 3
              i32.shr_u
              i32.rotl
              i32.and
              i32.store offset=1032
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
            i32.load offset=1440
            i32.ne
            br_if $B8
            i32.const 0
            get_local $l0
            i32.store offset=1432
            get_local $l1
            return
          end
          i32.const 4095
          set_local $p2
        end
        i32.const 0
        i32.const -1
        i32.store offset=1472
        i32.const 0
        get_local $p2
        i32.store offset=1480
        get_local $l1
        return
      end
      block $B22
        block $B23
          block $B24
            block $B25
              block $B26
                get_local $l0
                i32.const 255
                i32.gt_u
                br_if $B26
                get_local $l0
                i32.const 3
                i32.shr_u
                tee_local $p0
                i32.const 3
                i32.shl
                i32.const 1040
                i32.add
                set_local $l0
                i32.const 0
                i32.load offset=1032
                tee_local $p1
                i32.const 1
                get_local $p0
                i32.const 31
                i32.and
                i32.shl
                tee_local $p0
                i32.and
                i32.eqz
                br_if $B25
                get_local $l0
                i32.const 8
                i32.add
                set_local $p1
                get_local $l0
                i32.load offset=8
                set_local $p0
                br $B24
              end
              get_local $p2
              get_local $l0
              call $dlmalloc::dlmalloc::Dlmalloc::insert_large_chunk::hfbbc13dfd26ec0ad
              i32.const 0
              i32.const 0
              i32.load offset=1480
              i32.const -1
              i32.add
              tee_local $p2
              i32.store offset=1480
              get_local $p2
              br_if $B0
              i32.const 0
              i32.load offset=1464
              tee_local $l0
              i32.eqz
              br_if $B23
              i32.const 0
              set_local $p2
              loop $L27
                get_local $p2
                i32.const 1
                i32.add
                set_local $p2
                get_local $l0
                i32.load offset=8
                tee_local $l0
                br_if $L27
              end
              get_local $p2
              i32.const 4095
              get_local $p2
              i32.const 4095
              i32.gt_u
              select
              set_local $p2
              br $B22
            end
            i32.const 0
            get_local $p1
            get_local $p0
            i32.or
            i32.store offset=1032
            get_local $l0
            i32.const 8
            i32.add
            set_local $p1
            get_local $l0
            set_local $p0
          end
          get_local $p1
          get_local $p2
          i32.store
          get_local $p0
          get_local $p2
          i32.store offset=12
          get_local $p2
          get_local $l0
          i32.store offset=12
          get_local $p2
          get_local $p0
          i32.store offset=8
          get_local $l1
          return
        end
        i32.const 4095
        set_local $p2
      end
      i32.const 0
      get_local $p2
      i32.store offset=1480
    end
    get_local $l1)
  (func $dlmalloc::dlmalloc::Dlmalloc::unlink_large_chunk::hf712b91716024651 (type $t1) (param $p0 i32)
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
            i32.const 1304
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
    i32.load offset=1036
    i32.const -2
    get_local $l3
    i32.rotl
    i32.and
    i32.store offset=1036)
  (func $dlmalloc::dlmalloc::Dlmalloc::insert_large_chunk::hfbbc13dfd26ec0ad (type $t2) (param $p0 i32) (param $p1 i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32)
    i32.const 0
    set_local $l0
    block $B0
      get_local $p1
      i32.const 8
      i32.shr_u
      tee_local $l1
      i32.eqz
      br_if $B0
      i32.const 31
      set_local $l0
      get_local $p1
      i32.const 16777215
      i32.gt_u
      br_if $B0
      get_local $p1
      i32.const 38
      get_local $l1
      i32.clz
      tee_local $l0
      i32.sub
      i32.const 31
      i32.and
      i32.shr_u
      i32.const 1
      i32.and
      i32.const 31
      get_local $l0
      i32.sub
      i32.const 1
      i32.shl
      i32.or
      set_local $l0
    end
    get_local $p0
    get_local $l0
    i32.store offset=28
    get_local $p0
    i64.const 0
    i64.store offset=16 align=4
    get_local $l0
    i32.const 2
    i32.shl
    i32.const 1304
    i32.add
    set_local $l1
    block $B1
      block $B2
        block $B3
          block $B4
            i32.const 0
            i32.load offset=1036
            tee_local $l2
            i32.const 1
            get_local $l0
            i32.const 31
            i32.and
            i32.shl
            tee_local $l3
            i32.and
            i32.eqz
            br_if $B4
            get_local $l1
            i32.load
            tee_local $l2
            i32.load offset=4
            i32.const -8
            i32.and
            get_local $p1
            i32.ne
            br_if $B3
            get_local $l2
            set_local $l0
            br $B2
          end
          get_local $l1
          get_local $p0
          i32.store
          i32.const 0
          get_local $l2
          get_local $l3
          i32.or
          i32.store offset=1036
          get_local $p0
          get_local $l1
          i32.store offset=24
          get_local $p0
          get_local $p0
          i32.store offset=8
          get_local $p0
          get_local $p0
          i32.store offset=12
          return
        end
        get_local $p1
        i32.const 0
        i32.const 25
        get_local $l0
        i32.const 1
        i32.shr_u
        i32.sub
        i32.const 31
        i32.and
        get_local $l0
        i32.const 31
        i32.eq
        select
        i32.shl
        set_local $l1
        loop $L5
          get_local $l2
          get_local $l1
          i32.const 29
          i32.shr_u
          i32.const 4
          i32.and
          i32.add
          i32.const 16
          i32.add
          tee_local $l3
          i32.load
          tee_local $l0
          i32.eqz
          br_if $B1
          get_local $l1
          i32.const 1
          i32.shl
          set_local $l1
          get_local $l0
          set_local $l2
          get_local $l0
          i32.load offset=4
          i32.const -8
          i32.and
          get_local $p1
          i32.ne
          br_if $L5
        end
      end
      get_local $l0
      i32.load offset=8
      tee_local $l1
      get_local $p0
      i32.store offset=12
      get_local $l0
      get_local $p0
      i32.store offset=8
      get_local $p0
      get_local $l0
      i32.store offset=12
      get_local $p0
      get_local $l1
      i32.store offset=8
      get_local $p0
      i32.const 0
      i32.store offset=24
      return
    end
    get_local $l3
    get_local $p0
    i32.store
    get_local $p0
    get_local $l2
    i32.store offset=24
    get_local $p0
    get_local $p0
    i32.store offset=12
    get_local $p0
    get_local $p0
    i32.store offset=8)
  (func $sum_four (export "sum_four") (type $t3) (result i32)
    (local $l0 i32) (local $l1 i32) (local $l2 i32) (local $l3 i32) (local $l4 i32) (local $l5 i32) (local $l6 i32) (local $l7 i64) (local $l8 i32)
    i32.const 0
    i32.load offset=1032
    tee_local $l0
    i32.const 3
    i32.shr_u
    set_local $l1
    block $B0
      block $B1
        block $B2
          block $B3
            block $B4
              block $B5
                block $B6
                  block $B7
                    block $B8
                      block $B9
                        block $B10
                          block $B11
                            block $B12
                              block $B13
                                block $B14
                                  block $B15
                                    block $B16
                                      block $B17
                                        block $B18
                                          block $B19
                                            block $B20
                                              block $B21
                                                block $B22
                                                  block $B23
                                                    block $B24
                                                      block $B25
                                                        block $B26
                                                          get_local $l0
                                                          i32.const 24
                                                          i32.and
                                                          i32.eqz
                                                          br_if $B26
                                                          i32.const 4
                                                          get_local $l1
                                                          i32.const 1
                                                          i32.and
                                                          i32.sub
                                                          tee_local $l2
                                                          i32.const 3
                                                          i32.shl
                                                          tee_local $l3
                                                          i32.const 1048
                                                          i32.add
                                                          i32.load
                                                          tee_local $l4
                                                          i32.const 8
                                                          i32.add
                                                          set_local $l1
                                                          get_local $l4
                                                          i32.load offset=8
                                                          tee_local $l5
                                                          get_local $l3
                                                          i32.const 1040
                                                          i32.add
                                                          tee_local $l3
                                                          i32.eq
                                                          br_if $B25
                                                          get_local $l5
                                                          get_local $l3
                                                          i32.store offset=12
                                                          get_local $l3
                                                          i32.const 8
                                                          i32.add
                                                          get_local $l5
                                                          i32.store
                                                          br $B24
                                                        end
                                                        i32.const 0
                                                        i32.load offset=1432
                                                        tee_local $l4
                                                        i32.const 23
                                                        i32.gt_u
                                                        br_if $B23
                                                        get_local $l1
                                                        i32.eqz
                                                        br_if $B22
                                                        get_local $l0
                                                        i32.const 0
                                                        get_local $l0
                                                        i32.const -16
                                                        i32.and
                                                        i32.sub
                                                        i32.and
                                                        i32.ctz
                                                        tee_local $l2
                                                        i32.const 3
                                                        i32.shl
                                                        tee_local $l5
                                                        i32.const 1048
                                                        i32.add
                                                        i32.load
                                                        tee_local $l4
                                                        i32.load offset=8
                                                        tee_local $l1
                                                        get_local $l5
                                                        i32.const 1040
                                                        i32.add
                                                        tee_local $l5
                                                        i32.eq
                                                        br_if $B21
                                                        get_local $l1
                                                        get_local $l5
                                                        i32.store offset=12
                                                        get_local $l5
                                                        i32.const 8
                                                        i32.add
                                                        get_local $l1
                                                        i32.store
                                                        br $B20
                                                      end
                                                      i32.const 0
                                                      get_local $l0
                                                      i32.const -2
                                                      get_local $l2
                                                      i32.rotl
                                                      i32.and
                                                      i32.store offset=1032
                                                    end
                                                    get_local $l4
                                                    get_local $l2
                                                    i32.const 3
                                                    i32.shl
                                                    tee_local $l0
                                                    i32.const 3
                                                    i32.or
                                                    i32.store offset=4
                                                    get_local $l4
                                                    get_local $l0
                                                    i32.add
                                                    tee_local $l0
                                                    get_local $l0
                                                    i32.load offset=4
                                                    i32.const 1
                                                    i32.or
                                                    i32.store offset=4
                                                    br $B0
                                                  end
                                                  i32.const 0
                                                  i32.load offset=1440
                                                  set_local $l1
                                                  block $B27
                                                    block $B28
                                                      get_local $l4
                                                      i32.const -24
                                                      i32.add
                                                      tee_local $l0
                                                      i32.const 16
                                                      i32.ge_u
                                                      br_if $B28
                                                      i32.const 0
                                                      i32.const 0
                                                      i32.store offset=1440
                                                      i32.const 0
                                                      i32.const 0
                                                      i32.store offset=1432
                                                      get_local $l1
                                                      get_local $l4
                                                      i32.const 3
                                                      i32.or
                                                      i32.store offset=4
                                                      get_local $l1
                                                      get_local $l4
                                                      i32.add
                                                      tee_local $l4
                                                      i32.const 4
                                                      i32.add
                                                      set_local $l0
                                                      get_local $l4
                                                      i32.load offset=4
                                                      i32.const 1
                                                      i32.or
                                                      set_local $l4
                                                      br $B27
                                                    end
                                                    i32.const 0
                                                    get_local $l0
                                                    i32.store offset=1432
                                                    i32.const 0
                                                    get_local $l1
                                                    i32.const 24
                                                    i32.add
                                                    i32.store offset=1440
                                                    get_local $l1
                                                    i32.const 28
                                                    i32.add
                                                    get_local $l0
                                                    i32.const 1
                                                    i32.or
                                                    i32.store
                                                    get_local $l1
                                                    get_local $l4
                                                    i32.add
                                                    get_local $l0
                                                    i32.store
                                                    get_local $l1
                                                    i32.const 4
                                                    i32.add
                                                    set_local $l0
                                                    i32.const 27
                                                    set_local $l4
                                                  end
                                                  get_local $l0
                                                  get_local $l4
                                                  i32.store
                                                  get_local $l1
                                                  i32.const 8
                                                  i32.add
                                                  set_local $l1
                                                  br $B0
                                                end
                                                i32.const 0
                                                i32.load offset=1036
                                                tee_local $l1
                                                i32.eqz
                                                br_if $B19
                                                get_local $l1
                                                i32.const 0
                                                get_local $l1
                                                i32.sub
                                                i32.and
                                                i32.ctz
                                                i32.const 2
                                                i32.shl
                                                i32.const 1304
                                                i32.add
                                                i32.load
                                                tee_local $l4
                                                i32.load offset=4
                                                i32.const -24
                                                i32.add
                                                i32.const -8
                                                i32.and
                                                set_local $l0
                                                get_local $l4
                                                set_local $l2
                                                block $B29
                                                  loop $L30
                                                    block $B31
                                                      get_local $l4
                                                      i32.load offset=16
                                                      tee_local $l1
                                                      br_if $B31
                                                      get_local $l4
                                                      i32.const 20
                                                      i32.add
                                                      i32.load
                                                      tee_local $l1
                                                      i32.eqz
                                                      br_if $B29
                                                    end
                                                    get_local $l1
                                                    i32.load offset=4
                                                    i32.const -24
                                                    i32.add
                                                    i32.const -8
                                                    i32.and
                                                    tee_local $l4
                                                    get_local $l0
                                                    get_local $l4
                                                    get_local $l0
                                                    i32.lt_u
                                                    tee_local $l4
                                                    select
                                                    set_local $l0
                                                    get_local $l1
                                                    get_local $l2
                                                    get_local $l4
                                                    select
                                                    set_local $l2
                                                    get_local $l1
                                                    set_local $l4
                                                    br $L30
                                                  end
                                                end
                                                get_local $l2
                                                call $dlmalloc::dlmalloc::Dlmalloc::unlink_large_chunk::hf712b91716024651
                                                get_local $l0
                                                i32.const 16
                                                i32.ge_u
                                                br_if $B18
                                                get_local $l2
                                                get_local $l0
                                                i32.const 24
                                                i32.add
                                                tee_local $l1
                                                i32.const 3
                                                i32.or
                                                i32.store offset=4
                                                get_local $l2
                                                get_local $l1
                                                i32.add
                                                tee_local $l1
                                                get_local $l1
                                                i32.load offset=4
                                                i32.const 1
                                                i32.or
                                                i32.store offset=4
                                                br $B13
                                              end
                                              i32.const 0
                                              get_local $l0
                                              i32.const -2
                                              get_local $l2
                                              i32.rotl
                                              i32.and
                                              i32.store offset=1032
                                            end
                                            get_local $l4
                                            i32.const 8
                                            i32.add
                                            set_local $l1
                                            get_local $l4
                                            i32.const 27
                                            i32.store offset=4
                                            get_local $l4
                                            get_local $l2
                                            i32.const 3
                                            i32.shl
                                            tee_local $l0
                                            i32.add
                                            get_local $l0
                                            i32.const -24
                                            i32.add
                                            tee_local $l2
                                            i32.store
                                            get_local $l4
                                            i32.const 28
                                            i32.add
                                            get_local $l2
                                            i32.const 1
                                            i32.or
                                            i32.store
                                            get_local $l4
                                            i32.const 24
                                            i32.add
                                            set_local $l5
                                            block $B32
                                              i32.const 0
                                              i32.load offset=1432
                                              tee_local $l0
                                              i32.eqz
                                              br_if $B32
                                              get_local $l0
                                              i32.const 3
                                              i32.shr_u
                                              tee_local $l3
                                              i32.const 3
                                              i32.shl
                                              i32.const 1040
                                              i32.add
                                              set_local $l4
                                              i32.const 0
                                              i32.load offset=1440
                                              set_local $l0
                                              block $B33
                                                block $B34
                                                  i32.const 0
                                                  i32.load offset=1032
                                                  tee_local $l6
                                                  i32.const 1
                                                  get_local $l3
                                                  i32.const 31
                                                  i32.and
                                                  i32.shl
                                                  tee_local $l3
                                                  i32.and
                                                  i32.eqz
                                                  br_if $B34
                                                  get_local $l4
                                                  i32.load offset=8
                                                  set_local $l3
                                                  br $B33
                                                end
                                                i32.const 0
                                                get_local $l6
                                                get_local $l3
                                                i32.or
                                                i32.store offset=1032
                                                get_local $l4
                                                set_local $l3
                                              end
                                              get_local $l4
                                              i32.const 8
                                              i32.add
                                              get_local $l0
                                              i32.store
                                              get_local $l3
                                              get_local $l0
                                              i32.store offset=12
                                              get_local $l0
                                              get_local $l4
                                              i32.store offset=12
                                              get_local $l0
                                              get_local $l3
                                              i32.store offset=8
                                            end
                                            i32.const 0
                                            get_local $l5
                                            i32.store offset=1440
                                            i32.const 0
                                            get_local $l2
                                            i32.store offset=1432
                                            br $B0
                                          end
                                          i32.const 0
                                          i32.load offset=1436
                                          tee_local $l1
                                          i32.const 24
                                          i32.le_u
                                          br_if $B17
                                          i32.const 0
                                          get_local $l1
                                          i32.const -24
                                          i32.add
                                          tee_local $l0
                                          i32.store offset=1436
                                          i32.const 0
                                          i32.const 0
                                          i32.load offset=1444
                                          tee_local $l1
                                          i32.const 24
                                          i32.add
                                          i32.store offset=1444
                                          get_local $l1
                                          i32.const 27
                                          i32.store offset=4
                                          get_local $l1
                                          i32.const 28
                                          i32.add
                                          get_local $l0
                                          i32.const 1
                                          i32.or
                                          i32.store
                                          get_local $l1
                                          i32.const 8
                                          i32.add
                                          set_local $l1
                                          br $B0
                                        end
                                        get_local $l2
                                        i32.const 27
                                        i32.store offset=4
                                        get_local $l2
                                        i32.const 28
                                        i32.add
                                        get_local $l0
                                        i32.const 1
                                        i32.or
                                        i32.store
                                        get_local $l2
                                        i32.const 24
                                        i32.add
                                        tee_local $l5
                                        get_local $l0
                                        i32.add
                                        get_local $l0
                                        i32.store
                                        i32.const 0
                                        i32.load offset=1432
                                        tee_local $l1
                                        i32.eqz
                                        br_if $B14
                                        get_local $l1
                                        i32.const 3
                                        i32.shr_u
                                        tee_local $l3
                                        i32.const 3
                                        i32.shl
                                        i32.const 1040
                                        i32.add
                                        set_local $l4
                                        i32.const 0
                                        i32.load offset=1440
                                        set_local $l1
                                        i32.const 0
                                        i32.load offset=1032
                                        tee_local $l6
                                        i32.const 1
                                        get_local $l3
                                        i32.const 31
                                        i32.and
                                        i32.shl
                                        tee_local $l3
                                        i32.and
                                        i32.eqz
                                        br_if $B16
                                        get_local $l4
                                        i32.load offset=8
                                        set_local $l3
                                        br $B15
                                      end
                                      i32.const 1
                                      grow_memory
                                      tee_local $l1
                                      i32.const -1
                                      i32.eq
                                      br_if $B1
                                      get_local $l1
                                      i32.const 16
                                      i32.shl
                                      tee_local $l2
                                      i32.eqz
                                      br_if $B1
                                      i32.const 0
                                      i32.const 0
                                      i32.load offset=1448
                                      i32.const 65536
                                      i32.add
                                      tee_local $l1
                                      i32.store offset=1448
                                      i32.const 0
                                      i32.const 0
                                      i32.load offset=1452
                                      tee_local $l0
                                      get_local $l1
                                      get_local $l1
                                      get_local $l0
                                      i32.lt_u
                                      select
                                      i32.store offset=1452
                                      i32.const 0
                                      i32.load offset=1444
                                      tee_local $l0
                                      i32.eqz
                                      br_if $B12
                                      i32.const 1456
                                      set_local $l1
                                      loop $L35
                                        get_local $l1
                                        i32.load
                                        tee_local $l4
                                        get_local $l1
                                        i32.load offset=4
                                        tee_local $l5
                                        i32.add
                                        get_local $l2
                                        i32.eq
                                        br_if $B11
                                        get_local $l1
                                        i32.load offset=8
                                        tee_local $l1
                                        br_if $L35
                                        br $B10
                                      end
                                    end
                                    i32.const 0
                                    get_local $l6
                                    get_local $l3
                                    i32.or
                                    i32.store offset=1032
                                    get_local $l4
                                    set_local $l3
                                  end
                                  get_local $l4
                                  i32.const 8
                                  i32.add
                                  get_local $l1
                                  i32.store
                                  get_local $l3
                                  get_local $l1
                                  i32.store offset=12
                                  get_local $l1
                                  get_local $l4
                                  i32.store offset=12
                                  get_local $l1
                                  get_local $l3
                                  i32.store offset=8
                                end
                                i32.const 0
                                get_local $l5
                                i32.store offset=1440
                                i32.const 0
                                get_local $l0
                                i32.store offset=1432
                              end
                              get_local $l2
                              i32.const 8
                              i32.add
                              set_local $l1
                              br $B0
                            end
                            block $B36
                              block $B37
                                i32.const 0
                                i32.load offset=1476
                                tee_local $l1
                                i32.eqz
                                br_if $B37
                                get_local $l1
                                get_local $l2
                                i32.le_u
                                br_if $B36
                              end
                              i32.const 0
                              get_local $l2
                              i32.store offset=1476
                            end
                            i32.const 0
                            set_local $l1
                            i32.const 0
                            i32.const 65536
                            i32.store offset=1460
                            i32.const 0
                            get_local $l2
                            i32.store offset=1456
                            i32.const 0
                            i32.const 4095
                            i32.store offset=1480
                            i32.const 0
                            i32.const 0
                            i32.store offset=1468
                            loop $L38
                              get_local $l1
                              i32.const 1048
                              i32.add
                              get_local $l1
                              i32.const 1040
                              i32.add
                              tee_local $l0
                              i32.store
                              get_local $l1
                              i32.const 1052
                              i32.add
                              get_local $l0
                              i32.store
                              get_local $l1
                              i32.const 8
                              i32.add
                              tee_local $l1
                              i32.const 256
                              i32.ne
                              br_if $L38
                            end
                            i32.const 0
                            i32.const 65496
                            i32.store offset=1436
                            i32.const 0
                            get_local $l2
                            i32.store offset=1444
                            get_local $l2
                            i32.const 65497
                            i32.store offset=4
                            get_local $l2
                            i32.const 40
                            i32.store offset=65500
                            i32.const 0
                            i32.const 2097152
                            i32.store offset=1472
                            br $B9
                          end
                          get_local $l1
                          i32.load offset=12
                          br_if $B10
                          get_local $l2
                          get_local $l0
                          i32.le_u
                          br_if $B10
                          get_local $l4
                          get_local $l0
                          i32.gt_u
                          br_if $B10
                          get_local $l1
                          i32.const 4
                          i32.add
                          get_local $l5
                          i32.const 65536
                          i32.add
                          i32.store
                          i32.const 0
                          i32.load offset=1444
                          tee_local $l1
                          i32.const 15
                          i32.add
                          i32.const -8
                          i32.and
                          tee_local $l0
                          i32.const -8
                          i32.add
                          tee_local $l4
                          i32.const 0
                          i32.load offset=1436
                          i32.const 65536
                          i32.add
                          tee_local $l2
                          get_local $l0
                          get_local $l1
                          i32.const 8
                          i32.add
                          i32.sub
                          i32.sub
                          tee_local $l0
                          i32.const 1
                          i32.or
                          i32.store offset=4
                          i32.const 0
                          i32.const 2097152
                          i32.store offset=1472
                          i32.const 0
                          get_local $l4
                          i32.store offset=1444
                          i32.const 0
                          get_local $l0
                          i32.store offset=1436
                          get_local $l1
                          get_local $l2
                          i32.add
                          i32.const 40
                          i32.store offset=4
                          br $B9
                        end
                        i32.const 0
                        i32.const 0
                        i32.load offset=1476
                        tee_local $l1
                        get_local $l2
                        get_local $l1
                        get_local $l2
                        i32.lt_u
                        select
                        i32.store offset=1476
                        get_local $l2
                        i32.const 65536
                        i32.add
                        set_local $l4
                        i32.const 1456
                        set_local $l1
                        block $B39
                          block $B40
                            block $B41
                              block $B42
                                block $B43
                                  loop $L44
                                    get_local $l1
                                    i32.load
                                    get_local $l4
                                    i32.eq
                                    br_if $B43
                                    get_local $l1
                                    i32.load offset=8
                                    tee_local $l1
                                    br_if $L44
                                    br $B42
                                  end
                                end
                                get_local $l1
                                i32.load offset=12
                                i32.eqz
                                br_if $B41
                              end
                              i32.const 1456
                              set_local $l1
                              block $B45
                                loop $L46
                                  block $B47
                                    get_local $l1
                                    i32.load
                                    tee_local $l4
                                    get_local $l0
                                    i32.gt_u
                                    br_if $B47
                                    get_local $l4
                                    get_local $l1
                                    i32.load offset=4
                                    i32.add
                                    tee_local $l4
                                    get_local $l0
                                    i32.gt_u
                                    br_if $B45
                                  end
                                  get_local $l1
                                  i32.load offset=8
                                  set_local $l1
                                  br $L46
                                end
                              end
                              get_local $l2
                              i32.const 65497
                              i32.store offset=4
                              get_local $l2
                              i32.const 40
                              i32.store offset=65500
                              get_local $l0
                              get_local $l4
                              i32.const -32
                              i32.add
                              i32.const -8
                              i32.and
                              i32.const -8
                              i32.add
                              tee_local $l1
                              get_local $l1
                              get_local $l0
                              i32.const 16
                              i32.add
                              i32.lt_u
                              select
                              tee_local $l5
                              i32.const 27
                              i32.store offset=4
                              i32.const 0
                              i32.const 65496
                              i32.store offset=1436
                              i32.const 0
                              get_local $l2
                              i32.store offset=1444
                              i32.const 0
                              i32.const 2097152
                              i32.store offset=1472
                              i32.const 0
                              i64.load offset=1456 align=4
                              set_local $l7
                              get_local $l5
                              i32.const 16
                              i32.add
                              i32.const 0
                              i64.load offset=1464 align=4
                              i64.store align=4
                              get_local $l5
                              get_local $l7
                              i64.store offset=8 align=4
                              i32.const 0
                              i32.const 65536
                              i32.store offset=1460
                              i32.const 0
                              get_local $l2
                              i32.store offset=1456
                              i32.const 0
                              get_local $l5
                              i32.const 8
                              i32.add
                              i32.store offset=1464
                              i32.const 0
                              i32.const 0
                              i32.store offset=1468
                              get_local $l5
                              i32.const 28
                              i32.add
                              set_local $l1
                              loop $L48
                                get_local $l1
                                i32.const 7
                                i32.store
                                get_local $l4
                                get_local $l1
                                i32.const 4
                                i32.add
                                tee_local $l1
                                i32.gt_u
                                br_if $L48
                              end
                              get_local $l5
                              get_local $l0
                              i32.eq
                              br_if $B9
                              get_local $l5
                              get_local $l5
                              i32.load offset=4
                              i32.const -2
                              i32.and
                              i32.store offset=4
                              get_local $l0
                              get_local $l5
                              get_local $l0
                              i32.sub
                              tee_local $l1
                              i32.const 1
                              i32.or
                              i32.store offset=4
                              get_local $l5
                              get_local $l1
                              i32.store
                              block $B49
                                get_local $l1
                                i32.const 255
                                i32.gt_u
                                br_if $B49
                                get_local $l1
                                i32.const 3
                                i32.shr_u
                                tee_local $l4
                                i32.const 3
                                i32.shl
                                i32.const 1040
                                i32.add
                                set_local $l1
                                i32.const 0
                                i32.load offset=1032
                                tee_local $l2
                                i32.const 1
                                get_local $l4
                                i32.const 31
                                i32.and
                                i32.shl
                                tee_local $l4
                                i32.and
                                i32.eqz
                                br_if $B40
                                get_local $l1
                                i32.load offset=8
                                set_local $l4
                                br $B39
                              end
                              get_local $l0
                              get_local $l1
                              call $dlmalloc::dlmalloc::Dlmalloc::insert_large_chunk::hfbbc13dfd26ec0ad
                              br $B9
                            end
                            get_local $l1
                            get_local $l2
                            i32.store
                            get_local $l1
                            get_local $l1
                            i32.load offset=4
                            i32.const 65536
                            i32.add
                            i32.store offset=4
                            get_local $l2
                            i32.const 27
                            i32.store offset=4
                            get_local $l2
                            i32.const 24
                            i32.add
                            set_local $l0
                            get_local $l4
                            get_local $l2
                            i32.sub
                            i32.const -24
                            i32.add
                            set_local $l1
                            i32.const 0
                            i32.load offset=1444
                            get_local $l4
                            i32.eq
                            br_if $B8
                            i32.const 0
                            i32.load offset=1440
                            get_local $l4
                            i32.eq
                            br_if $B7
                            get_local $l2
                            i32.load offset=65540
                            tee_local $l5
                            i32.const 3
                            i32.and
                            i32.const 1
                            i32.ne
                            br_if $B3
                            get_local $l5
                            i32.const -8
                            i32.and
                            tee_local $l3
                            i32.const 255
                            i32.gt_u
                            br_if $B6
                            get_local $l2
                            i32.load offset=65548
                            tee_local $l6
                            get_local $l2
                            i32.load offset=65544
                            tee_local $l8
                            i32.eq
                            br_if $B5
                            get_local $l8
                            get_local $l6
                            i32.store offset=12
                            get_local $l6
                            get_local $l8
                            i32.store offset=8
                            br $B4
                          end
                          i32.const 0
                          get_local $l2
                          get_local $l4
                          i32.or
                          i32.store offset=1032
                          get_local $l1
                          set_local $l4
                        end
                        get_local $l1
                        i32.const 8
                        i32.add
                        get_local $l0
                        i32.store
                        get_local $l4
                        get_local $l0
                        i32.store offset=12
                        get_local $l0
                        get_local $l1
                        i32.store offset=12
                        get_local $l0
                        get_local $l4
                        i32.store offset=8
                      end
                      i32.const 0
                      i32.load offset=1436
                      tee_local $l1
                      i32.const 25
                      i32.lt_u
                      br_if $B1
                      i32.const 0
                      get_local $l1
                      i32.const -24
                      i32.add
                      tee_local $l0
                      i32.store offset=1436
                      i32.const 0
                      i32.const 0
                      i32.load offset=1444
                      tee_local $l1
                      i32.const 24
                      i32.add
                      i32.store offset=1444
                      get_local $l1
                      i32.const 27
                      i32.store offset=4
                      get_local $l1
                      i32.const 28
                      i32.add
                      get_local $l0
                      i32.const 1
                      i32.or
                      i32.store
                      get_local $l1
                      i32.const 8
                      i32.add
                      set_local $l1
                      br $B0
                    end
                    i32.const 0
                    get_local $l0
                    i32.store offset=1444
                    i32.const 0
                    i32.const 0
                    i32.load offset=1436
                    get_local $l1
                    i32.add
                    tee_local $l1
                    i32.store offset=1436
                    get_local $l2
                    get_local $l1
                    i32.const 1
                    i32.or
                    i32.store offset=28
                    br $B2
                  end
                  get_local $l2
                  i32.const 0
                  i32.load offset=1432
                  get_local $l1
                  i32.add
                  tee_local $l1
                  i32.const 1
                  i32.or
                  i32.store offset=28
                  i32.const 0
                  get_local $l0
                  i32.store offset=1440
                  i32.const 0
                  get_local $l1
                  i32.store offset=1432
                  get_local $l0
                  get_local $l1
                  i32.add
                  get_local $l1
                  i32.store
                  br $B2
                end
                get_local $l4
                call $dlmalloc::dlmalloc::Dlmalloc::unlink_large_chunk::hf712b91716024651
                br $B4
              end
              i32.const 0
              i32.const 0
              i32.load offset=1032
              i32.const -2
              get_local $l5
              i32.const 3
              i32.shr_u
              i32.rotl
              i32.and
              i32.store offset=1032
            end
            get_local $l3
            get_local $l1
            i32.add
            set_local $l1
            get_local $l4
            get_local $l3
            i32.add
            set_local $l4
          end
          get_local $l4
          get_local $l4
          i32.load offset=4
          i32.const -2
          i32.and
          i32.store offset=4
          get_local $l2
          get_local $l1
          i32.const 1
          i32.or
          i32.store offset=28
          get_local $l0
          get_local $l1
          i32.add
          get_local $l1
          i32.store
          block $B50
            block $B51
              block $B52
                get_local $l1
                i32.const 255
                i32.gt_u
                br_if $B52
                get_local $l1
                i32.const 3
                i32.shr_u
                tee_local $l4
                i32.const 3
                i32.shl
                i32.const 1040
                i32.add
                set_local $l1
                i32.const 0
                i32.load offset=1032
                tee_local $l5
                i32.const 1
                get_local $l4
                i32.const 31
                i32.and
                i32.shl
                tee_local $l4
                i32.and
                i32.eqz
                br_if $B51
                get_local $l1
                i32.const 8
                i32.add
                set_local $l5
                get_local $l1
                i32.load offset=8
                set_local $l4
                br $B50
              end
              get_local $l0
              get_local $l1
              call $dlmalloc::dlmalloc::Dlmalloc::insert_large_chunk::hfbbc13dfd26ec0ad
              br $B2
            end
            i32.const 0
            get_local $l5
            get_local $l4
            i32.or
            i32.store offset=1032
            get_local $l1
            i32.const 8
            i32.add
            set_local $l5
            get_local $l1
            set_local $l4
          end
          get_local $l5
          get_local $l0
          i32.store
          get_local $l4
          get_local $l0
          i32.store offset=12
          get_local $l2
          get_local $l1
          i32.store offset=36
          get_local $l2
          get_local $l4
          i32.store offset=32
        end
        get_local $l2
        i32.const 8
        i32.add
        set_local $l1
        br $B0
      end
      i32.const 16
      i32.const 4
      call $rust_oom
      unreachable
    end
    get_local $l1
    i64.const 8589934593
    i64.store align=4
    get_local $l1
    i64.const 17179869187
    i64.store offset=8 align=4
    get_local $l1
    i32.const 4
    i32.const 4
    call $sum_vec)
  (func $rust_oom (type $t2) (param $p0 i32) (param $p1 i32)
    unreachable
    unreachable)
  (table $T0 1 1 anyfunc)
  (memory $memory (export "memory") 17)
  (data (i32.const 1024) "\00\00\00\00\00")
  (data (i32.const 1032) "\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"))

