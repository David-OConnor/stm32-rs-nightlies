#[doc = "Register `UR2` reader"]
pub struct R(crate::R<UR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UR2` writer"]
pub struct W(crate::W<UR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UR2_SPEC>;
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
impl From<crate::W<UR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BORH` reader - BOR_LVL Brownout Reset Threshold Level"]
pub type BORH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BORH` writer - BOR_LVL Brownout Reset Threshold Level"]
pub type BORH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `BOOT_ADD0` reader - Boot Address 0"]
pub type BOOT_ADD0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BOOT_ADD0` writer - Boot Address 0"]
pub type BOOT_ADD0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UR2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:1 - BOR_LVL Brownout Reset Threshold Level"]
    #[inline(always)]
    pub fn borh(&self) -> BORH_R {
        BORH_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:31 - Boot Address 0"]
    #[inline(always)]
    pub fn boot_add0(&self) -> BOOT_ADD0_R {
        BOOT_ADD0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - BOR_LVL Brownout Reset Threshold Level"]
    #[inline(always)]
    #[must_use]
    pub fn borh(&mut self) -> BORH_W<0> {
        BORH_W::new(self)
    }
    #[doc = "Bits 16:31 - Boot Address 0"]
    #[inline(always)]
    #[must_use]
    pub fn boot_add0(&mut self) -> BOOT_ADD0_W<16> {
        BOOT_ADD0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG user register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur2](index.html) module"]
pub struct UR2_SPEC;
impl crate::RegisterSpec for UR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur2::R](R) reader structure"]
impl crate::Readable for UR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ur2::W](W) writer structure"]
impl crate::Writable for UR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UR2 to value 0"]
impl crate::Resettable for UR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
