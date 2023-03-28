#[doc = "Register `OTG_HFIR` reader"]
pub struct R(crate::R<OTG_HFIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HFIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HFIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HFIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_HFIR` writer"]
pub struct W(crate::W<OTG_HFIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_HFIR_SPEC>;
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
impl From<crate::W<OTG_HFIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_HFIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRIVL` reader - FRIVL"]
pub type FRIVL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FRIVL` writer - FRIVL"]
pub type FRIVL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_HFIR_SPEC, u16, u16, 16, O>;
#[doc = "Field `RLDCTRL` reader - RLDCTRL"]
pub type RLDCTRL_R = crate::BitReader<bool>;
#[doc = "Field `RLDCTRL` writer - RLDCTRL"]
pub type RLDCTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HFIR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - FRIVL"]
    #[inline(always)]
    pub fn frivl(&self) -> FRIVL_R {
        FRIVL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - RLDCTRL"]
    #[inline(always)]
    pub fn rldctrl(&self) -> RLDCTRL_R {
        RLDCTRL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - FRIVL"]
    #[inline(always)]
    #[must_use]
    pub fn frivl(&mut self) -> FRIVL_W<0> {
        FRIVL_W::new(self)
    }
    #[doc = "Bit 16 - RLDCTRL"]
    #[inline(always)]
    #[must_use]
    pub fn rldctrl(&mut self) -> RLDCTRL_W<16> {
        RLDCTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register stores the frame interval information for the current speed to which the OTG controller has enumerated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hfir](index.html) module"]
pub struct OTG_HFIR_SPEC;
impl crate::RegisterSpec for OTG_HFIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_hfir::R](R) reader structure"]
impl crate::Readable for OTG_HFIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_hfir::W](W) writer structure"]
impl crate::Writable for OTG_HFIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_HFIR to value 0xea60"]
impl crate::Resettable for OTG_HFIR_SPEC {
    const RESET_VALUE: Self::Ux = 0xea60;
}
