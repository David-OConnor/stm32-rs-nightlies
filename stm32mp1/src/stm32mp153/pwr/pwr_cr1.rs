#[doc = "Register `PWR_CR1` reader"]
pub struct R(crate::R<PWR_CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_CR1` writer"]
pub struct W(crate::W<PWR_CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_CR1_SPEC>;
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
impl From<crate::W<PWR_CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPDS` reader - LPDS"]
pub type LPDS_R = crate::BitReader<bool>;
#[doc = "Field `LPDS` writer - LPDS"]
pub type LPDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR1_SPEC, bool, O>;
#[doc = "Field `LPCFG` reader - LPCFG"]
pub type LPCFG_R = crate::BitReader<bool>;
#[doc = "Field `LPCFG` writer - LPCFG"]
pub type LPCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR1_SPEC, bool, O>;
#[doc = "Field `LVDS` reader - LVDS"]
pub type LVDS_R = crate::BitReader<bool>;
#[doc = "Field `LVDS` writer - LVDS"]
pub type LVDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR1_SPEC, bool, O>;
#[doc = "Field `PVDEN` reader - PVDEN"]
pub type PVDEN_R = crate::BitReader<bool>;
#[doc = "Field `PVDEN` writer - PVDEN"]
pub type PVDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR1_SPEC, bool, O>;
#[doc = "Field `PLS` reader - PLS"]
pub type PLS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLS` writer - PLS"]
pub type PLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_CR1_SPEC, u8, u8, 3, O>;
#[doc = "Field `DBP` reader - DBP"]
pub type DBP_R = crate::BitReader<bool>;
#[doc = "Field `DBP` writer - DBP"]
pub type DBP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR1_SPEC, bool, O>;
#[doc = "Field `AVDEN` reader - AVDEN"]
pub type AVDEN_R = crate::BitReader<bool>;
#[doc = "Field `AVDEN` writer - AVDEN"]
pub type AVDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_CR1_SPEC, bool, O>;
#[doc = "Field `ALS` reader - ALS"]
pub type ALS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALS` writer - ALS"]
pub type ALS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWR_CR1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - LPDS"]
    #[inline(always)]
    pub fn lpds(&self) -> LPDS_R {
        LPDS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPCFG"]
    #[inline(always)]
    pub fn lpcfg(&self) -> LPCFG_R {
        LPCFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LVDS"]
    #[inline(always)]
    pub fn lvds(&self) -> LVDS_R {
        LVDS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - PVDEN"]
    #[inline(always)]
    pub fn pvden(&self) -> PVDEN_R {
        PVDEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - PLS"]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - DBP"]
    #[inline(always)]
    pub fn dbp(&self) -> DBP_R {
        DBP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - AVDEN"]
    #[inline(always)]
    pub fn avden(&self) -> AVDEN_R {
        AVDEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - ALS"]
    #[inline(always)]
    pub fn als(&self) -> ALS_R {
        ALS_R::new(((self.bits >> 17) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LPDS"]
    #[inline(always)]
    #[must_use]
    pub fn lpds(&mut self) -> LPDS_W<0> {
        LPDS_W::new(self)
    }
    #[doc = "Bit 1 - LPCFG"]
    #[inline(always)]
    #[must_use]
    pub fn lpcfg(&mut self) -> LPCFG_W<1> {
        LPCFG_W::new(self)
    }
    #[doc = "Bit 2 - LVDS"]
    #[inline(always)]
    #[must_use]
    pub fn lvds(&mut self) -> LVDS_W<2> {
        LVDS_W::new(self)
    }
    #[doc = "Bit 4 - PVDEN"]
    #[inline(always)]
    #[must_use]
    pub fn pvden(&mut self) -> PVDEN_W<4> {
        PVDEN_W::new(self)
    }
    #[doc = "Bits 5:7 - PLS"]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PLS_W<5> {
        PLS_W::new(self)
    }
    #[doc = "Bit 8 - DBP"]
    #[inline(always)]
    #[must_use]
    pub fn dbp(&mut self) -> DBP_W<8> {
        DBP_W::new(self)
    }
    #[doc = "Bit 16 - AVDEN"]
    #[inline(always)]
    #[must_use]
    pub fn avden(&mut self) -> AVDEN_W<16> {
        AVDEN_W::new(self)
    }
    #[doc = "Bits 17:18 - ALS"]
    #[inline(always)]
    #[must_use]
    pub fn als(&mut self) -> ALS_W<17> {
        ALS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset on any system reset. This register provides write access security when enabled by TZEN register bit in Section10: Reset and clock control (RCC). When security is enabled a non-secure write access generates a bus error. Secure and non-secure read accesses are granted and return the register value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_cr1](index.html) module"]
pub struct PWR_CR1_SPEC;
impl crate::RegisterSpec for PWR_CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_cr1::R](R) reader structure"]
impl crate::Readable for PWR_CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_cr1::W](W) writer structure"]
impl crate::Writable for PWR_CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_CR1 to value 0"]
impl crate::Resettable for PWR_CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
