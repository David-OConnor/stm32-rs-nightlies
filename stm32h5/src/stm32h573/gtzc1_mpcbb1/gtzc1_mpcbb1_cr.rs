#[doc = "Register `GTZC1_MPCBB1_CR` reader"]
pub struct R(crate::R<GTZC1_MPCBB1_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTZC1_MPCBB1_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTZC1_MPCBB1_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTZC1_MPCBB1_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTZC1_MPCBB1_CR` writer"]
pub struct W(crate::W<GTZC1_MPCBB1_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTZC1_MPCBB1_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GTZC1_MPCBB1_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTZC1_MPCBB1_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GLOCK` reader - lock the control register of the MPCBB until next reset This bit is cleared by default and once set, it can not be reset until system reset."]
pub type GLOCK_R = crate::BitReader<bool>;
#[doc = "Field `GLOCK` writer - lock the control register of the MPCBB until next reset This bit is cleared by default and once set, it can not be reset until system reset."]
pub type GLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB1_CR_SPEC, bool, O>;
#[doc = "Field `INVSECSTATE` reader - SRAMx clocks security state This bit is used to define the internal SRAMs clocks control in RCC as secure or not."]
pub type INVSECSTATE_R = crate::BitReader<bool>;
#[doc = "Field `INVSECSTATE` writer - SRAMx clocks security state This bit is used to define the internal SRAMs clocks control in RCC as secure or not."]
pub type INVSECSTATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB1_CR_SPEC, bool, O>;
#[doc = "Field `SRWILADIS` reader - secure read/write illegal access disable This bit disables the detection of an illegal access when a secure read/write transaction access a non-secure blocks of the block-based SRAM (secure fetch on non-secure block is always considered illegal)."]
pub type SRWILADIS_R = crate::BitReader<bool>;
#[doc = "Field `SRWILADIS` writer - secure read/write illegal access disable This bit disables the detection of an illegal access when a secure read/write transaction access a non-secure blocks of the block-based SRAM (secure fetch on non-secure block is always considered illegal)."]
pub type SRWILADIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, GTZC1_MPCBB1_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - lock the control register of the MPCBB until next reset This bit is cleared by default and once set, it can not be reset until system reset."]
    #[inline(always)]
    pub fn glock(&self) -> GLOCK_R {
        GLOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 30 - SRAMx clocks security state This bit is used to define the internal SRAMs clocks control in RCC as secure or not."]
    #[inline(always)]
    pub fn invsecstate(&self) -> INVSECSTATE_R {
        INVSECSTATE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - secure read/write illegal access disable This bit disables the detection of an illegal access when a secure read/write transaction access a non-secure blocks of the block-based SRAM (secure fetch on non-secure block is always considered illegal)."]
    #[inline(always)]
    pub fn srwiladis(&self) -> SRWILADIS_R {
        SRWILADIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock the control register of the MPCBB until next reset This bit is cleared by default and once set, it can not be reset until system reset."]
    #[inline(always)]
    #[must_use]
    pub fn glock(&mut self) -> GLOCK_W<0> {
        GLOCK_W::new(self)
    }
    #[doc = "Bit 30 - SRAMx clocks security state This bit is used to define the internal SRAMs clocks control in RCC as secure or not."]
    #[inline(always)]
    #[must_use]
    pub fn invsecstate(&mut self) -> INVSECSTATE_W<30> {
        INVSECSTATE_W::new(self)
    }
    #[doc = "Bit 31 - secure read/write illegal access disable This bit disables the detection of an illegal access when a secure read/write transaction access a non-secure blocks of the block-based SRAM (secure fetch on non-secure block is always considered illegal)."]
    #[inline(always)]
    #[must_use]
    pub fn srwiladis(&mut self) -> SRWILADIS_W<31> {
        SRWILADIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GTZC1 SRAM1 MPCBB control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtzc1_mpcbb1_cr](index.html) module"]
pub struct GTZC1_MPCBB1_CR_SPEC;
impl crate::RegisterSpec for GTZC1_MPCBB1_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtzc1_mpcbb1_cr::R](R) reader structure"]
impl crate::Readable for GTZC1_MPCBB1_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtzc1_mpcbb1_cr::W](W) writer structure"]
impl crate::Writable for GTZC1_MPCBB1_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTZC1_MPCBB1_CR to value 0"]
impl crate::Resettable for GTZC1_MPCBB1_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
