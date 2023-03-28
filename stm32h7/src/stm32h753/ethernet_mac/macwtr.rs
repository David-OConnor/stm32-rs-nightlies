#[doc = "Register `MACWTR` reader"]
pub struct R(crate::R<MACWTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACWTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACWTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACWTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACWTR` writer"]
pub struct W(crate::W<MACWTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACWTR_SPEC>;
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
impl From<crate::W<MACWTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACWTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WTO` reader - Watchdog Timeout"]
pub type WTO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WTO` writer - Watchdog Timeout"]
pub type WTO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACWTR_SPEC, u8, u8, 4, O>;
#[doc = "Field `PWE` reader - Programmable Watchdog Enable"]
pub type PWE_R = crate::BitReader<bool>;
#[doc = "Field `PWE` writer - Programmable Watchdog Enable"]
pub type PWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MACWTR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Watchdog Timeout"]
    #[inline(always)]
    pub fn wto(&self) -> WTO_R {
        WTO_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Programmable Watchdog Enable"]
    #[inline(always)]
    pub fn pwe(&self) -> PWE_R {
        PWE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Watchdog Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn wto(&mut self) -> WTO_W<0> {
        WTO_W::new(self)
    }
    #[doc = "Bit 8 - Programmable Watchdog Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwe(&mut self) -> PWE_W<8> {
        PWE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog timeout register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macwtr](index.html) module"]
pub struct MACWTR_SPEC;
impl crate::RegisterSpec for MACWTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macwtr::R](R) reader structure"]
impl crate::Readable for MACWTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macwtr::W](W) writer structure"]
impl crate::Writable for MACWTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACWTR to value 0"]
impl crate::Resettable for MACWTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
