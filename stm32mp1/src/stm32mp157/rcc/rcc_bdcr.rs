#[doc = "Register `RCC_BDCR` reader"]
pub struct R(crate::R<RCC_BDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_BDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_BDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_BDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_BDCR` writer"]
pub struct W(crate::W<RCC_BDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_BDCR_SPEC>;
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
impl From<crate::W<RCC_BDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_BDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSEON` reader - LSEON"]
pub type LSEON_R = crate::BitReader<bool>;
#[doc = "Field `LSEON` writer - LSEON"]
pub type LSEON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BDCR_SPEC, bool, O>;
#[doc = "Field `LSEBYP` reader - LSEBYP"]
pub type LSEBYP_R = crate::BitReader<bool>;
#[doc = "Field `LSEBYP` writer - LSEBYP"]
pub type LSEBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BDCR_SPEC, bool, O>;
#[doc = "Field `LSERDY` reader - LSERDY"]
pub type LSERDY_R = crate::BitReader<bool>;
#[doc = "Field `DIGBYP` reader - DIGBYP"]
pub type DIGBYP_R = crate::BitReader<bool>;
#[doc = "Field `LSEDRV` reader - LSEDRV"]
pub type LSEDRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LSEDRV` writer - LSEDRV"]
pub type LSEDRV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_BDCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `LSECSSON` reader - LSECSSON"]
pub type LSECSSON_R = crate::BitReader<bool>;
#[doc = "Field `LSECSSON` writer - LSECSSON"]
pub type LSECSSON_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BDCR_SPEC, bool, O>;
#[doc = "Field `LSECSSD` reader - LSECSSD"]
pub type LSECSSD_R = crate::BitReader<bool>;
#[doc = "Field `RTCSRC` reader - RTCSRC"]
pub type RTCSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTCCKEN` reader - RTCCKEN"]
pub type RTCCKEN_R = crate::BitReader<bool>;
#[doc = "Field `RTCCKEN` writer - RTCCKEN"]
pub type RTCCKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BDCR_SPEC, bool, O>;
#[doc = "Field `VSWRST` reader - VSWRST"]
pub type VSWRST_R = crate::BitReader<bool>;
#[doc = "Field `VSWRST` writer - VSWRST"]
pub type VSWRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_BDCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - LSEON"]
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSEBYP"]
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LSERDY"]
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DIGBYP"]
    #[inline(always)]
    pub fn digbyp(&self) -> DIGBYP_R {
        DIGBYP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - LSEDRV"]
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - LSECSSON"]
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LSECSSD"]
    #[inline(always)]
    pub fn lsecssd(&self) -> LSECSSD_R {
        LSECSSD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:17 - RTCSRC"]
    #[inline(always)]
    pub fn rtcsrc(&self) -> RTCSRC_R {
        RTCSRC_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - RTCCKEN"]
    #[inline(always)]
    pub fn rtccken(&self) -> RTCCKEN_R {
        RTCCKEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 31 - VSWRST"]
    #[inline(always)]
    pub fn vswrst(&self) -> VSWRST_R {
        VSWRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSEON"]
    #[inline(always)]
    #[must_use]
    pub fn lseon(&mut self) -> LSEON_W<0> {
        LSEON_W::new(self)
    }
    #[doc = "Bit 1 - LSEBYP"]
    #[inline(always)]
    #[must_use]
    pub fn lsebyp(&mut self) -> LSEBYP_W<1> {
        LSEBYP_W::new(self)
    }
    #[doc = "Bits 4:5 - LSEDRV"]
    #[inline(always)]
    #[must_use]
    pub fn lsedrv(&mut self) -> LSEDRV_W<4> {
        LSEDRV_W::new(self)
    }
    #[doc = "Bit 8 - LSECSSON"]
    #[inline(always)]
    #[must_use]
    pub fn lsecsson(&mut self) -> LSECSSON_W<8> {
        LSECSSON_W::new(self)
    }
    #[doc = "Bit 20 - RTCCKEN"]
    #[inline(always)]
    #[must_use]
    pub fn rtccken(&mut self) -> RTCCKEN_W<20> {
        RTCCKEN_W::new(self)
    }
    #[doc = "Bit 31 - VSWRST"]
    #[inline(always)]
    #[must_use]
    pub fn vswrst(&mut self) -> VSWRST_W<31> {
        VSWRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the LSE function. Wait states are inserted in case of successive write accesses to this register. The number of wait states may be up to 7 cycles of AHB4 clock.After a system reset, the register RCC_BDCR is write-protected. In order to modify this register, the DBP bit in the PWR control register 1 (PWR_CR1) has to be set to . Bits of RCC_BDCR register are only reset after a backup domain reset: nreset_vsw (see Section10.3.6: Backup domain reset). Any other internal or external reset will not have any effect on these bits.This register is located into the VSW domain. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_bdcr](index.html) module"]
pub struct RCC_BDCR_SPEC;
impl crate::RegisterSpec for RCC_BDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_bdcr::R](R) reader structure"]
impl crate::Readable for RCC_BDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_bdcr::W](W) writer structure"]
impl crate::Writable for RCC_BDCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_BDCR to value 0x20"]
impl crate::Resettable for RCC_BDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
