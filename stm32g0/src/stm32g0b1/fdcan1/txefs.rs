#[doc = "Register `TXEFS` reader"]
pub struct R(crate::R<TXEFS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXEFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXEFS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXEFS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EFFL` reader - Event FIFO fill level Number of elements stored in Tx event FIFO, range 0 to 3."]
pub type EFFL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EFGI` reader - Event FIFO get index Tx Event FIFO read index pointer, range 0 to 3."]
pub type EFGI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EFPI` reader - Event FIFO put index Tx Event FIFO write index pointer, range 0 to 3."]
pub type EFPI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EFF` reader - Event FIFO full"]
pub type EFF_R = crate::BitReader<bool>;
#[doc = "Field `TEFL` reader - Tx Event FIFO element lost This bit is a copy of interrupt flag IR\\[TEFL\\]. When IR\\[TEFL\\]
is reset, this bit is also reset. 0 No Tx event FIFO element lost 1 Tx event FIFO element lost, also set after write attempt to Tx Event FIFO of size 0."]
pub type TEFL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:2 - Event FIFO fill level Number of elements stored in Tx event FIFO, range 0 to 3."]
    #[inline(always)]
    pub fn effl(&self) -> EFFL_R {
        EFFL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - Event FIFO get index Tx Event FIFO read index pointer, range 0 to 3."]
    #[inline(always)]
    pub fn efgi(&self) -> EFGI_R {
        EFGI_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Event FIFO put index Tx Event FIFO write index pointer, range 0 to 3."]
    #[inline(always)]
    pub fn efpi(&self) -> EFPI_R {
        EFPI_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - Event FIFO full"]
    #[inline(always)]
    pub fn eff(&self) -> EFF_R {
        EFF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Tx Event FIFO element lost This bit is a copy of interrupt flag IR\\[TEFL\\]. When IR\\[TEFL\\]
is reset, this bit is also reset. 0 No Tx event FIFO element lost 1 Tx event FIFO element lost, also set after write attempt to Tx Event FIFO of size 0."]
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "FDCAN Tx event FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txefs](index.html) module"]
pub struct TXEFS_SPEC;
impl crate::RegisterSpec for TXEFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txefs::R](R) reader structure"]
impl crate::Readable for TXEFS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TXEFS to value 0"]
impl crate::Resettable for TXEFS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
