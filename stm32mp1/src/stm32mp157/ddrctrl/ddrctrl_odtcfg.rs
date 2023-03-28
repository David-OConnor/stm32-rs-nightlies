#[doc = "Register `DDRCTRL_ODTCFG` reader"]
pub struct R(crate::R<DDRCTRL_ODTCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_ODTCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_ODTCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_ODTCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_ODTCFG` writer"]
pub struct W(crate::W<DDRCTRL_ODTCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_ODTCFG_SPEC>;
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
impl From<crate::W<DDRCTRL_ODTCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_ODTCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RD_ODT_DELAY` reader - RD_ODT_DELAY"]
pub type RD_ODT_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RD_ODT_DELAY` writer - RD_ODT_DELAY"]
pub type RD_ODT_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_ODTCFG_SPEC, u8, u8, 5, O>;
#[doc = "Field `RD_ODT_HOLD` reader - RD_ODT_HOLD"]
pub type RD_ODT_HOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RD_ODT_HOLD` writer - RD_ODT_HOLD"]
pub type RD_ODT_HOLD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_ODTCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `WR_ODT_DELAY` reader - WR_ODT_DELAY"]
pub type WR_ODT_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WR_ODT_DELAY` writer - WR_ODT_DELAY"]
pub type WR_ODT_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_ODTCFG_SPEC, u8, u8, 5, O>;
#[doc = "Field `WR_ODT_HOLD` reader - WR_ODT_HOLD"]
pub type WR_ODT_HOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WR_ODT_HOLD` writer - WR_ODT_HOLD"]
pub type WR_ODT_HOLD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_ODTCFG_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 2:6 - RD_ODT_DELAY"]
    #[inline(always)]
    pub fn rd_odt_delay(&self) -> RD_ODT_DELAY_R {
        RD_ODT_DELAY_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - RD_ODT_HOLD"]
    #[inline(always)]
    pub fn rd_odt_hold(&self) -> RD_ODT_HOLD_R {
        RD_ODT_HOLD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - WR_ODT_DELAY"]
    #[inline(always)]
    pub fn wr_odt_delay(&self) -> WR_ODT_DELAY_R {
        WR_ODT_DELAY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - WR_ODT_HOLD"]
    #[inline(always)]
    pub fn wr_odt_hold(&self) -> WR_ODT_HOLD_R {
        WR_ODT_HOLD_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:6 - RD_ODT_DELAY"]
    #[inline(always)]
    #[must_use]
    pub fn rd_odt_delay(&mut self) -> RD_ODT_DELAY_W<2> {
        RD_ODT_DELAY_W::new(self)
    }
    #[doc = "Bits 8:11 - RD_ODT_HOLD"]
    #[inline(always)]
    #[must_use]
    pub fn rd_odt_hold(&mut self) -> RD_ODT_HOLD_W<8> {
        RD_ODT_HOLD_W::new(self)
    }
    #[doc = "Bits 16:20 - WR_ODT_DELAY"]
    #[inline(always)]
    #[must_use]
    pub fn wr_odt_delay(&mut self) -> WR_ODT_DELAY_W<16> {
        WR_ODT_DELAY_W::new(self)
    }
    #[doc = "Bits 24:27 - WR_ODT_HOLD"]
    #[inline(always)]
    #[must_use]
    pub fn wr_odt_hold(&mut self) -> WR_ODT_HOLD_W<24> {
        WR_ODT_HOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL ODT configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_odtcfg](index.html) module"]
pub struct DDRCTRL_ODTCFG_SPEC;
impl crate::RegisterSpec for DDRCTRL_ODTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_odtcfg::R](R) reader structure"]
impl crate::Readable for DDRCTRL_ODTCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_odtcfg::W](W) writer structure"]
impl crate::Writable for DDRCTRL_ODTCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_ODTCFG to value 0x0400_0400"]
impl crate::Resettable for DDRCTRL_ODTCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400_0400;
}
