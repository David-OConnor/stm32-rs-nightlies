#[doc = "Register `DDRCTRL_ADDRMAP10` reader"]
pub struct R(crate::R<DDRCTRL_ADDRMAP10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_ADDRMAP10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_ADDRMAP10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_ADDRMAP10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_ADDRMAP10` writer"]
pub struct W(crate::W<DDRCTRL_ADDRMAP10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_ADDRMAP10_SPEC>;
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
impl From<crate::W<DDRCTRL_ADDRMAP10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_ADDRMAP10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRMAP_ROW_B6` reader - ADDRMAP_ROW_B6"]
pub type ADDRMAP_ROW_B6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRMAP_ROW_B6` writer - ADDRMAP_ROW_B6"]
pub type ADDRMAP_ROW_B6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_ADDRMAP10_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADDRMAP_ROW_B7` reader - ADDRMAP_ROW_B7"]
pub type ADDRMAP_ROW_B7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRMAP_ROW_B7` writer - ADDRMAP_ROW_B7"]
pub type ADDRMAP_ROW_B7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_ADDRMAP10_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADDRMAP_ROW_B8` reader - ADDRMAP_ROW_B8"]
pub type ADDRMAP_ROW_B8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRMAP_ROW_B8` writer - ADDRMAP_ROW_B8"]
pub type ADDRMAP_ROW_B8_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_ADDRMAP10_SPEC, u8, u8, 4, O>;
#[doc = "Field `ADDRMAP_ROW_B9` reader - ADDRMAP_ROW_B9"]
pub type ADDRMAP_ROW_B9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDRMAP_ROW_B9` writer - ADDRMAP_ROW_B9"]
pub type ADDRMAP_ROW_B9_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DDRCTRL_ADDRMAP10_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - ADDRMAP_ROW_B6"]
    #[inline(always)]
    pub fn addrmap_row_b6(&self) -> ADDRMAP_ROW_B6_R {
        ADDRMAP_ROW_B6_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - ADDRMAP_ROW_B7"]
    #[inline(always)]
    pub fn addrmap_row_b7(&self) -> ADDRMAP_ROW_B7_R {
        ADDRMAP_ROW_B7_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - ADDRMAP_ROW_B8"]
    #[inline(always)]
    pub fn addrmap_row_b8(&self) -> ADDRMAP_ROW_B8_R {
        ADDRMAP_ROW_B8_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - ADDRMAP_ROW_B9"]
    #[inline(always)]
    pub fn addrmap_row_b9(&self) -> ADDRMAP_ROW_B9_R {
        ADDRMAP_ROW_B9_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADDRMAP_ROW_B6"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b6(&mut self) -> ADDRMAP_ROW_B6_W<0> {
        ADDRMAP_ROW_B6_W::new(self)
    }
    #[doc = "Bits 8:11 - ADDRMAP_ROW_B7"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b7(&mut self) -> ADDRMAP_ROW_B7_W<8> {
        ADDRMAP_ROW_B7_W::new(self)
    }
    #[doc = "Bits 16:19 - ADDRMAP_ROW_B8"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b8(&mut self) -> ADDRMAP_ROW_B8_W<16> {
        ADDRMAP_ROW_B8_W::new(self)
    }
    #[doc = "Bits 24:27 - ADDRMAP_ROW_B9"]
    #[inline(always)]
    #[must_use]
    pub fn addrmap_row_b9(&mut self) -> ADDRMAP_ROW_B9_W<24> {
        ADDRMAP_ROW_B9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL address map register 10\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_addrmap10](index.html) module"]
pub struct DDRCTRL_ADDRMAP10_SPEC;
impl crate::RegisterSpec for DDRCTRL_ADDRMAP10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_addrmap10::R](R) reader structure"]
impl crate::Readable for DDRCTRL_ADDRMAP10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_addrmap10::W](W) writer structure"]
impl crate::Writable for DDRCTRL_ADDRMAP10_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDRCTRL_ADDRMAP10 to value 0"]
impl crate::Resettable for DDRCTRL_ADDRMAP10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
