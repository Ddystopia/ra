///Register `PIPEBUF` reader
pub type R = crate::R<PIPEBUF_SPEC>;
///Register `PIPEBUF` writer
pub type W = crate::W<PIPEBUF_SPEC>;
/**Buffer NumberThese bits specify the FIFO buffer number of the selected pipe (04h to 87h).

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BUFNMB_A {
    ///0: specify the FIFO buffer number of the selected pipe (04h to 87h).
    BUFNMB = 0,
}
impl From<BUFNMB_A> for u8 {
    #[inline(always)]
    fn from(variant: BUFNMB_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BUFNMB_A {
    type Ux = u8;
}
impl crate::IsEnum for BUFNMB_A {}
///Field `BUFNMB` reader - Buffer NumberThese bits specify the FIFO buffer number of the selected pipe (04h to 87h).
pub type BUFNMB_R = crate::FieldReader<BUFNMB_A>;
impl BUFNMB_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BUFNMB_A {
        match self.bits {
            _ => BUFNMB_A::BUFNMB,
        }
    }
    ///specify the FIFO buffer number of the selected pipe (04h to 87h).
    #[inline(always)]
    pub fn is_bufnmb(&self) -> bool {
        matches!(self.variant(), BUFNMB_A::BUFNMB)
    }
}
///Field `BUFNMB` writer - Buffer NumberThese bits specify the FIFO buffer number of the selected pipe (04h to 87h).
pub type BUFNMB_W<'a, REG> = crate::FieldWriter<'a, REG, 8, BUFNMB_A, crate::Safe>;
impl<'a, REG> BUFNMB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///specify the FIFO buffer number of the selected pipe (04h to 87h).
    #[inline(always)]
    pub fn bufnmb(self) -> &'a mut crate::W<REG> {
        self.variant(BUFNMB_A::BUFNMB)
    }
}
/**Buffer Size 00h: 64 bytes 01h: 128 bytes : 1Fh: 2 Kbytes

Value on reset: 0*/
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BUFSIZE_A {
    ///0: ( BUFSIZE + 1 ) x 64 kbytes
    BUFSIZE = 0,
}
impl From<BUFSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: BUFSIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BUFSIZE_A {
    type Ux = u8;
}
impl crate::IsEnum for BUFSIZE_A {}
///Field `BUFSIZE` reader - Buffer Size 00h: 64 bytes 01h: 128 bytes : 1Fh: 2 Kbytes
pub type BUFSIZE_R = crate::FieldReader<BUFSIZE_A>;
impl BUFSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BUFSIZE_A {
        match self.bits {
            _ => BUFSIZE_A::BUFSIZE,
        }
    }
    ///( BUFSIZE + 1 ) x 64 kbytes
    #[inline(always)]
    pub fn is_bufsize(&self) -> bool {
        matches!(self.variant(), BUFSIZE_A::BUFSIZE)
    }
}
///Field `BUFSIZE` writer - Buffer Size 00h: 64 bytes 01h: 128 bytes : 1Fh: 2 Kbytes
pub type BUFSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 5, BUFSIZE_A, crate::Safe>;
impl<'a, REG> BUFSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///( BUFSIZE + 1 ) x 64 kbytes
    #[inline(always)]
    pub fn bufsize(self) -> &'a mut crate::W<REG> {
        self.variant(BUFSIZE_A::BUFSIZE)
    }
}
impl R {
    ///Bits 0:7 - Buffer NumberThese bits specify the FIFO buffer number of the selected pipe (04h to 87h).
    #[inline(always)]
    pub fn bufnmb(&self) -> BUFNMB_R {
        BUFNMB_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 10:14 - Buffer Size 00h: 64 bytes 01h: 128 bytes : 1Fh: 2 Kbytes
    #[inline(always)]
    pub fn bufsize(&self) -> BUFSIZE_R {
        BUFSIZE_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:7 - Buffer NumberThese bits specify the FIFO buffer number of the selected pipe (04h to 87h).
    #[inline(always)]
    pub fn bufnmb(&mut self) -> BUFNMB_W<PIPEBUF_SPEC> {
        BUFNMB_W::new(self, 0)
    }
    ///Bits 10:14 - Buffer Size 00h: 64 bytes 01h: 128 bytes : 1Fh: 2 Kbytes
    #[inline(always)]
    pub fn bufsize(&mut self) -> BUFSIZE_W<PIPEBUF_SPEC> {
        BUFSIZE_W::new(self, 10)
    }
}
/**Pipe Buffer Register

You can [`read`](crate::Reg::read) this register and get [`pipebuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipebuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PIPEBUF_SPEC;
impl crate::RegisterSpec for PIPEBUF_SPEC {
    type Ux = u16;
}
///`read()` method returns [`pipebuf::R`](R) reader structure
impl crate::Readable for PIPEBUF_SPEC {}
///`write(|w| ..)` method takes [`pipebuf::W`](W) writer structure
impl crate::Writable for PIPEBUF_SPEC {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PIPEBUF to value 0
impl crate::Resettable for PIPEBUF_SPEC {}
