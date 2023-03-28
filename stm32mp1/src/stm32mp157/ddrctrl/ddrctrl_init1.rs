#[doc = "Register `DDRCTRL_INIT1` reader"]
pub struct R(crate::R<DDRCTRL_INIT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_INIT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_INIT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_INIT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_INIT1` writer"]
pub struct W(crate::W<DDRCTRL_INIT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_INIT1_SPEC>;
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
impl From<crate::W<DDRCTRL_INIT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_INIT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRE_OCD_X32` reader - PRE_OCD_X32"]
pub type PRE_OCD_X32_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRE_OCD_X32` writer - PRE_OCD_X32"]
pub type PRE_OCD_X32_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_INIT1_SPEC, u8, u8, 4, O>;
#[doc = "Field `DRAM_RSTN_X1024` reader - DRAM_RSTN_X1024"]
pub type DRAM_RSTN_X1024_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DRAM_RSTN_X1024` writer - DRAM_RSTN_X1024"]
pub type DRAM_RSTN_X1024_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_INIT1_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:3 - PRE_OCD_X32"]
    #[inline(always)]
    pub fn pre_ocd_x32(&self) -> PRE_OCD_X32_R {
        PRE_OCD_X32_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:24 - DRAM_RSTN_X1024"]
    #[inline(always)]
    pub fn dram_rstn_x1024(&self) -> DRAM_RSTN_X1024_R {
        DRAM_RSTN_X1024_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - PRE_OCD_X32"]
    #[inline(always)]
    #[must_use]
    pub fn pre_ocd_x32(&mut self) -> PRE_OCD_X32_W<0> {
        PRE_OCD_X32_W::new(self)
    }
    #[doc = "Bits 16:24 - DRAM_RSTN_X1024"]
    #[inline(always)]
    #[must_use]
    pub fn dram_rstn_x1024(&mut self) -> DRAM_RSTN_X1024_W<16> {
        DRAM_RSTN_X1024_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL SDRAM initialization register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_init1](index.html) module"]
pub struct DDRCTRL_INIT1_SPEC;
impl crate::RegisterSpec for DDRCTRL_INIT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_init1::R](R) reader structure"]
impl crate::Readable for DDRCTRL_INIT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_init1::W](W) writer structure"]
impl crate::Writable for DDRCTRL_INIT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_INIT1 to value 0"]
impl crate::Resettable for DDRCTRL_INIT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
