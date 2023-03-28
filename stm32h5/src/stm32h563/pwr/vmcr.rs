#[doc = "Register `VMCR` reader"]
pub struct R(crate::R<VMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VMCR` writer"]
pub struct W(crate::W<VMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VMCR_SPEC>;
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
impl From<crate::W<VMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PVDE` reader - PVD enable"]
pub type PVDE_R = crate::BitReader<bool>;
#[doc = "Field `PVDE` writer - PVD enable"]
pub type PVDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCR_SPEC, bool, O>;
#[doc = "Field `PLS` reader - programmable voltage detector (PVD) level selection These bits select the voltage threshold detected by the PVD."]
pub type PLS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLS` writer - programmable voltage detector (PVD) level selection These bits select the voltage threshold detected by the PVD."]
pub type PLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VMCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `AVDEN` reader - peripheral voltage monitor on V<sub>DDA</sub> enable"]
pub type AVDEN_R = crate::BitReader<bool>;
#[doc = "Field `AVDEN` writer - peripheral voltage monitor on V<sub>DDA</sub> enable"]
pub type AVDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMCR_SPEC, bool, O>;
#[doc = "Field `ALS` reader - analog voltage detector (AVD) level selection These bits select the voltage threshold detected by the AVD."]
pub type ALS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALS` writer - analog voltage detector (AVD) level selection These bits select the voltage threshold detected by the AVD."]
pub type ALS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VMCR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - PVD enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - programmable voltage detector (PVD) level selection These bits select the voltage threshold detected by the PVD."]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 8 - peripheral voltage monitor on V<sub>DDA</sub> enable"]
    #[inline(always)]
    pub fn avden(&self) -> AVDEN_R {
        AVDEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - analog voltage detector (AVD) level selection These bits select the voltage threshold detected by the AVD."]
    #[inline(always)]
    pub fn als(&self) -> ALS_R {
        ALS_R::new(((self.bits >> 9) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PVD enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvde(&mut self) -> PVDE_W<0> {
        PVDE_W::new(self)
    }
    #[doc = "Bits 1:3 - programmable voltage detector (PVD) level selection These bits select the voltage threshold detected by the PVD."]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PLS_W<1> {
        PLS_W::new(self)
    }
    #[doc = "Bit 8 - peripheral voltage monitor on V<sub>DDA</sub> enable"]
    #[inline(always)]
    #[must_use]
    pub fn avden(&mut self) -> AVDEN_W<8> {
        AVDEN_W::new(self)
    }
    #[doc = "Bits 9:10 - analog voltage detector (AVD) level selection These bits select the voltage threshold detected by the AVD."]
    #[inline(always)]
    #[must_use]
    pub fn als(&mut self) -> ALS_W<9> {
        ALS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWR voltage monitor control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vmcr](index.html) module"]
pub struct VMCR_SPEC;
impl crate::RegisterSpec for VMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vmcr::R](R) reader structure"]
impl crate::Readable for VMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vmcr::W](W) writer structure"]
impl crate::Writable for VMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VMCR to value 0"]
impl crate::Resettable for VMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
