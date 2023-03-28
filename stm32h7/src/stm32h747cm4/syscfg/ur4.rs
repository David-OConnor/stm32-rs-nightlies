#[doc = "Register `UR4` reader"]
pub struct R(crate::R<UR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UR4` writer"]
pub struct W(crate::W<UR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UR4_SPEC>;
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
impl From<crate::W<UR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCM4_ADD1` reader - Mass Erase Protected Area Disabled for bank 1"]
pub type BCM4_ADD1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BCM4_ADD1` writer - Mass Erase Protected Area Disabled for bank 1"]
pub type BCM4_ADD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UR4_SPEC, u16, u16, 16, O>;
#[doc = "Field `MEPAD_1` reader - Boot Cortex-M4 Address 1"]
pub type MEPAD_1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - Mass Erase Protected Area Disabled for bank 1"]
    #[inline(always)]
    pub fn bcm4_add1(&self) -> BCM4_ADD1_R {
        BCM4_ADD1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Boot Cortex-M4 Address 1"]
    #[inline(always)]
    pub fn mepad_1(&self) -> MEPAD_1_R {
        MEPAD_1_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Mass Erase Protected Area Disabled for bank 1"]
    #[inline(always)]
    #[must_use]
    pub fn bcm4_add1(&mut self) -> BCM4_ADD1_W<0> {
        BCM4_ADD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG user register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur4](index.html) module"]
pub struct UR4_SPEC;
impl crate::RegisterSpec for UR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur4::R](R) reader structure"]
impl crate::Readable for UR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ur4::W](W) writer structure"]
impl crate::Writable for UR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UR4 to value 0"]
impl crate::Resettable for UR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
