#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clears the Line Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLIFW_AW {
    #[doc = "1: Clears the LIF flag in the ISR register"]
    Clear = 1,
}
impl From<CLIFW_AW> for bool {
    #[inline(always)]
    fn from(variant: CLIFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLIF` writer - Clears the Line Interrupt Flag"]
pub type CLIF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ICR_SPEC, CLIFW_AW, O>;
impl<'a, const O: u8> CLIF_W<'a, O> {
    #[doc = "Clears the LIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CLIFW_AW::Clear)
    }
}
#[doc = "Clears the FIFO Underrun Interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFUIFW_AW {
    #[doc = "1: Clears the FUIF flag in the ISR register"]
    Clear = 1,
}
impl From<CFUIFW_AW> for bool {
    #[inline(always)]
    fn from(variant: CFUIFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFUIF` writer - Clears the FIFO Underrun Interrupt flag"]
pub type CFUIF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ICR_SPEC, CFUIFW_AW, O>;
impl<'a, const O: u8> CFUIF_W<'a, O> {
    #[doc = "Clears the FUIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CFUIFW_AW::Clear)
    }
}
#[doc = "Clears the Transfer Error Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTERRIFW_AW {
    #[doc = "1: Clears the TERRIF flag in the ISR register"]
    Clear = 1,
}
impl From<CTERRIFW_AW> for bool {
    #[inline(always)]
    fn from(variant: CTERRIFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTERRIF` writer - Clears the Transfer Error Interrupt Flag"]
pub type CTERRIF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ICR_SPEC, CTERRIFW_AW, O>;
impl<'a, const O: u8> CTERRIF_W<'a, O> {
    #[doc = "Clears the TERRIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTERRIFW_AW::Clear)
    }
}
#[doc = "Clears Register Reload Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRRIFW_AW {
    #[doc = "1: Clears the RRIF flag in the ISR register"]
    Clear = 1,
}
impl From<CRRIFW_AW> for bool {
    #[inline(always)]
    fn from(variant: CRRIFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRRIF` writer - Clears Register Reload Interrupt Flag"]
pub type CRRIF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ICR_SPEC, CRRIFW_AW, O>;
impl<'a, const O: u8> CRRIF_W<'a, O> {
    #[doc = "Clears the RRIF flag in the ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CRRIFW_AW::Clear)
    }
}
impl W {
    #[doc = "Bit 0 - Clears the Line Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn clif(&mut self) -> CLIF_W<0> {
        CLIF_W::new(self)
    }
    #[doc = "Bit 1 - Clears the FIFO Underrun Interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn cfuif(&mut self) -> CFUIF_W<1> {
        CFUIF_W::new(self)
    }
    #[doc = "Bit 2 - Clears the Transfer Error Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cterrif(&mut self) -> CTERRIF_W<2> {
        CTERRIF_W::new(self)
    }
    #[doc = "Bit 3 - Clears Register Reload Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn crrif(&mut self) -> CRRIF_W<3> {
        CRRIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0f;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
