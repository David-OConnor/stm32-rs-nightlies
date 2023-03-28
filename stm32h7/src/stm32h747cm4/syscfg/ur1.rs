#[doc = "Register `UR1` reader"]
pub struct R(crate::R<UR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UR1` writer"]
pub struct W(crate::W<UR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UR1_SPEC>;
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
impl From<crate::W<UR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCM4` reader - Boot Cortex-M4"]
pub type BCM4_R = crate::BitReader<bool>;
#[doc = "Field `BCM4` writer - Boot Cortex-M4"]
pub type BCM4_W<'a, const O: u8> = crate::BitWriter<'a, u32, UR1_SPEC, bool, O>;
#[doc = "Field `BCM7` reader - Boot Cortex-M7"]
pub type BCM7_R = crate::BitReader<bool>;
#[doc = "Field `BCM7` writer - Boot Cortex-M7"]
pub type BCM7_W<'a, const O: u8> = crate::BitWriter<'a, u32, UR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Boot Cortex-M4"]
    #[inline(always)]
    pub fn bcm4(&self) -> BCM4_R {
        BCM4_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Boot Cortex-M7"]
    #[inline(always)]
    pub fn bcm7(&self) -> BCM7_R {
        BCM7_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Boot Cortex-M4"]
    #[inline(always)]
    #[must_use]
    pub fn bcm4(&mut self) -> BCM4_W<0> {
        BCM4_W::new(self)
    }
    #[doc = "Bit 16 - Boot Cortex-M7"]
    #[inline(always)]
    #[must_use]
    pub fn bcm7(&mut self) -> BCM7_W<16> {
        BCM7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG user register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur1](index.html) module"]
pub struct UR1_SPEC;
impl crate::RegisterSpec for UR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur1::R](R) reader structure"]
impl crate::Readable for UR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ur1::W](W) writer structure"]
impl crate::Writable for UR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UR1 to value 0"]
impl crate::Resettable for UR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
