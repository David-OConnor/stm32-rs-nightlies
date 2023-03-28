#[doc = "Register `GTPR` reader"]
pub struct R(crate::R<GTPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTPR` writer"]
pub struct W(crate::W<GTPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTPR_SPEC>;
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
impl From<crate::W<GTPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSC` reader - Prescaler value In IrDA low-power and normal IrDA mode: PSC\\[7:0\\]
= IrDA Normal and Low-Power baud rate PSC\\[7:0\\]
is used to program the prescaler for dividing the USART source clock to achieve the low-power frequency: the source clock is divided by the value given in the register (8 significant bits): In Smartcard mode: PSC\\[4:0\\]
= Prescaler value PSC\\[4:0\\]
is used to program the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... ... This bitfield can only be written when the USART is disabled (UE = 0). Note: Bits \\[7:5\\]
must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to ‘0’ when the Smartcard and IrDA modes are not supported. Refer to ."]
pub type PSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSC` writer - Prescaler value In IrDA low-power and normal IrDA mode: PSC\\[7:0\\]
= IrDA Normal and Low-Power baud rate PSC\\[7:0\\]
is used to program the prescaler for dividing the USART source clock to achieve the low-power frequency: the source clock is divided by the value given in the register (8 significant bits): In Smartcard mode: PSC\\[4:0\\]
= Prescaler value PSC\\[4:0\\]
is used to program the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... ... This bitfield can only be written when the USART is disabled (UE = 0). Note: Bits \\[7:5\\]
must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to ‘0’ when the Smartcard and IrDA modes are not supported. Refer to ."]
pub type PSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTPR_SPEC, u8, u8, 8, O>;
#[doc = "Field `GT` reader - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE = 0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type GT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GT` writer - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE = 0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type GT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTPR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Prescaler value In IrDA low-power and normal IrDA mode: PSC\\[7:0\\]
= IrDA Normal and Low-Power baud rate PSC\\[7:0\\]
is used to program the prescaler for dividing the USART source clock to achieve the low-power frequency: the source clock is divided by the value given in the register (8 significant bits): In Smartcard mode: PSC\\[4:0\\]
= Prescaler value PSC\\[4:0\\]
is used to program the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... ... This bitfield can only be written when the USART is disabled (UE = 0). Note: Bits \\[7:5\\]
must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to ‘0’ when the Smartcard and IrDA modes are not supported. Refer to ."]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE = 0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn gt(&self) -> GT_R {
        GT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Prescaler value In IrDA low-power and normal IrDA mode: PSC\\[7:0\\]
= IrDA Normal and Low-Power baud rate PSC\\[7:0\\]
is used to program the prescaler for dividing the USART source clock to achieve the low-power frequency: the source clock is divided by the value given in the register (8 significant bits): In Smartcard mode: PSC\\[4:0\\]
= Prescaler value PSC\\[4:0\\]
is used to program the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... ... This bitfield can only be written when the USART is disabled (UE = 0). Note: Bits \\[7:5\\]
must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to ‘0’ when the Smartcard and IrDA modes are not supported. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<0> {
        PSC_W::new(self)
    }
    #[doc = "Bits 8:15 - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UE = 0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    #[must_use]
    pub fn gt(&mut self) -> GT_W<8> {
        GT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART guard time and prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtpr](index.html) module"]
pub struct GTPR_SPEC;
impl crate::RegisterSpec for GTPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtpr::R](R) reader structure"]
impl crate::Readable for GTPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtpr::W](W) writer structure"]
impl crate::Writable for GTPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTPR to value 0"]
impl crate::Resettable for GTPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
