#[doc = "Register `WKUPFR` reader"]
pub struct R(crate::R<WKUPFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WKUPFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WKUPFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WKUPFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WKUPFR` writer"]
pub struct W(crate::W<WKUPFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WKUPFR_SPEC>;
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
impl From<crate::W<WKUPFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WKUPFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKUPF1` reader - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type WKUPF1_R = crate::BitReader<bool>;
#[doc = "Field `WKUPF1` writer - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type WKUPF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKUPFR_SPEC, bool, O>;
#[doc = "Field `WKUPF2` reader - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type WKUPF2_R = crate::BitReader<bool>;
#[doc = "Field `WKUPF2` writer - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type WKUPF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKUPFR_SPEC, bool, O>;
#[doc = "Field `WKUPF3` reader - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type WKUPF3_R = crate::BitReader<bool>;
#[doc = "Field `WKUPF3` writer - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type WKUPF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKUPFR_SPEC, bool, O>;
#[doc = "Field `WKUPF4` reader - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type WKUPF4_R = crate::BitReader<bool>;
#[doc = "Field `WKUPF4` writer - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type WKUPF4_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKUPFR_SPEC, bool, O>;
#[doc = "Field `WKUPF5` reader - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type WKUPF5_R = crate::BitReader<bool>;
#[doc = "Field `WKUPF5` writer - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type WKUPF5_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKUPFR_SPEC, bool, O>;
#[doc = "Field `WKUPF6` reader - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type WKUPF6_R = crate::BitReader<bool>;
#[doc = "Field `WKUPF6` writer - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type WKUPF6_W<'a, const O: u8> = crate::BitWriter<'a, u32, WKUPFR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    pub fn wkupf1(&self) -> WKUPF1_R {
        WKUPF1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    pub fn wkupf2(&self) -> WKUPF2_R {
        WKUPF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    pub fn wkupf3(&self) -> WKUPF3_R {
        WKUPF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    pub fn wkupf4(&self) -> WKUPF4_R {
        WKUPF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    pub fn wkupf5(&self) -> WKUPF5_R {
        WKUPF5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    pub fn wkupf6(&self) -> WKUPF6_R {
        WKUPF6_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    #[must_use]
    pub fn wkupf1(&mut self) -> WKUPF1_W<0> {
        WKUPF1_W::new(self)
    }
    #[doc = "Bit 1 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    #[must_use]
    pub fn wkupf2(&mut self) -> WKUPF2_W<1> {
        WKUPF2_W::new(self)
    }
    #[doc = "Bit 2 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    #[must_use]
    pub fn wkupf3(&mut self) -> WKUPF3_W<2> {
        WKUPF3_W::new(self)
    }
    #[doc = "Bit 3 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    #[must_use]
    pub fn wkupf4(&mut self) -> WKUPF4_W<3> {
        WKUPF4_W::new(self)
    }
    #[doc = "Bit 4 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    #[must_use]
    pub fn wkupf5(&mut self) -> WKUPF5_W<4> {
        WKUPF5_W::new(self)
    }
    #[doc = "Bit 5 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    #[must_use]
    pub fn wkupf6(&mut self) -> WKUPF6_W<5> {
        WKUPF6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "reset only by system reset, not reset by wakeup from Standby mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wkupfr](index.html) module"]
pub struct WKUPFR_SPEC;
impl crate::RegisterSpec for WKUPFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wkupfr::R](R) reader structure"]
impl crate::Readable for WKUPFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wkupfr::W](W) writer structure"]
impl crate::Writable for WKUPFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WKUPFR to value 0"]
impl crate::Resettable for WKUPFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
